use super::types::Node;
use super::{utils, MappingsContext, SingleLineNode};
use std::rc::Rc;
use std::str;
use types::StringPtr;
use vlq;

#[derive(Clone, Debug)]
pub struct SourceNode {
    pub generated_code: String,
    pub original_source: Option<i32>,
    pub source: Option<i32>,
    pub starting_line: usize,
    pub number_of_lines: usize,
    pub ends_with_new_line: bool,
}

impl SourceNode {
    pub fn new(
        generated_code: String,
        source: Option<i32>,
        original_source: Option<i32>,
        starting_line: usize,
    ) -> Self {
        SourceNode {
            ends_with_new_line: generated_code.ends_with('\n'),
            number_of_lines: utils::number_of_lines(&generated_code),
            generated_code,
            original_source,
            source,
            starting_line,
        }
    }

    pub fn add_generated_code(&mut self, code: &str) {
        self.generated_code += code;
        self.number_of_lines += utils::number_of_lines(code);
        self.ends_with_new_line = code.ends_with('\n');
    }

    // pub fn map_generated_code(&self, fn_name: &str) -> SourceNode {
    // }

    pub fn merge(self, other_node: &Node) -> Result<Node, Node> {
        match other_node {
            Node::NSourceNode(n) => self.merge_source_node(n),
            Node::NSingleLineNode(n) => self.merge_single_line_node(n),
            _ => Err(Node::NSourceNode(self)),
        }
    }

    fn merge_source_node(mut self, other_node: &SourceNode) -> Result<Node, Node> {
        if self.source == other_node.source
            && self.ends_with_new_line
            && self.starting_line + self.number_of_lines == other_node.starting_line
        {
            self.generated_code += &other_node.generated_code;
            self.number_of_lines += other_node.number_of_lines;
            self.ends_with_new_line = other_node.ends_with_new_line;
            Ok(Node::NSourceNode(self))
        } else {
            Err(Node::NSourceNode(self))
        }
    }

    fn merge_single_line_node(mut self, other_node: &SingleLineNode) -> Result<Node, Node> {
        if self.source == other_node.source
            && self.ends_with_new_line
            && self.starting_line + self.number_of_lines == other_node.line
            && other_node.number_of_lines <= 1
        {
            self.add_single_line_node(other_node);
            Ok(Node::NSourceNode(self))
        } else {
            Err(Node::NSourceNode(self))
        }
    }

    fn add_single_line_node(&mut self, other_node: &SingleLineNode) -> &SourceNode {
        self.generated_code += &other_node.generated_code;
        self.number_of_lines += other_node.number_of_lines;
        self.ends_with_new_line = other_node.ends_with_new_line;
        self
    }

    pub fn get_generated_code(&self) -> &str {
        &self.generated_code
    }

    pub fn get_mappings(&self, mappings_context: &mut MappingsContext) -> String {
        let mut buf = Vec::<u8>::new();
        if self.generated_code.is_empty() {
            String::new()
        } else {
            let line_mapping = ";AACA";
            let lines = self.number_of_lines;
            let source_index = mappings_context.ensure_source(
                self.source.clone(),
                self.original_source.clone().map(|n| Node::NStringIdx(n)),
            );
            let mut mappings = String::from("A");
            if mappings_context.unfinished_generated_line != 0 {
                mappings = String::from(",");
                vlq::encode(mappings_context.unfinished_generated_line as i64, &mut buf).unwrap();
            }
            vlq::encode(
                source_index as i64 - mappings_context.current_source as i64,
                &mut buf,
            ).unwrap();
            vlq::encode(
                self.starting_line as i64 - mappings_context.current_original_line as i64,
                &mut buf,
            ).unwrap();
            buf.push(b'A');
            mappings += str::from_utf8(&buf).unwrap();
            buf.clear();

            mappings_context.current_source = source_index;
            mappings_context.current_original_line = self.starting_line + lines;
            mappings_context.current_original_line -= 1;

            let unfinished_generated_line = utils::get_unfinished_lines(&self.generated_code);
            mappings_context.unfinished_generated_line = unfinished_generated_line;
            if lines > 0 {
                mappings += &line_mapping.repeat(lines.saturating_sub(1));
            }

            if unfinished_generated_line == 0 {
                mappings += ";";
            } else {
                if lines != 0 {
                    mappings += line_mapping;
                }
                mappings_context.current_original_line += 1;
            }
            mappings
        }
    }

    pub fn get_normalized_nodes(&self) -> Vec<SingleLineNode> {
        let mut results = Vec::<SingleLineNode>::new();
        let mut current_line = self.starting_line;
        let mut lines = self.generated_code.split('\n').peekable();

        while let Some(line) = lines.next() {
            let line_code = if lines.peek().is_some() {
                String::from(line) + "\n"
            } else {
                if !self.ends_with_new_line {
                    String::from(line)
                } else {
                    break;
                }
            };

            results.push(SingleLineNode::new(
                line_code,
                self.source.clone(),
                self.original_source.clone(),
                current_line,
            ));
            current_line += 1;
        }
        results
    }
}
