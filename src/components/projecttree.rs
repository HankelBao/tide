use crate::core::{Message, MessageSender};
use crate::ui::{UIComponent, UISelector};
use crate::ui::{View};
use crate::terminal::DisplayLine;

use std::fs;
use std::sync::{Arc, Mutex};

pub struct ProjectNode {
    name: String,
    path: String,
    children: Vec<ProjectNode>,
}

impl ProjectNode {
    pub fn new(name: String, path: String) -> ProjectNode {
        ProjectNode {
            name,
            path,
            children: Vec::new(),
        }
    }

    pub fn build_children(&mut self) {
        self.children.clear();
        if let Ok(entries) = fs::read_dir(self.path.clone()) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let name = format!("{:?}", entry.file_name());
                    let path = format!("{:?}", entry.path());
                    self.children.push(ProjectNode::new(name, path));
                }
            }
        }
    }

    pub fn get_display_lines(&self, parent_indent_level: usize) -> Vec<DisplayLine> {
        let title_content = " ".repeat(parent_indent_level*4)+&self.name.clone();
        let first_displayline = DisplayLine::from(title_content, Vec::new());
        let mut display_lines = vec![first_displayline];
        for child in &self.children {
            let child_display_lines = child.get_display_lines(parent_indent_level+1);
            display_lines.extend(child_display_lines);
        }
        return display_lines
    }
}

pub struct ProjectTree {
    messagesender: MessageSender,
    view: Arc<Mutex<View>>,
    projectnode_root: ProjectNode,
    display_lines: Vec<DisplayLine>,
}

impl ProjectTree {
    pub fn new(messagesender: MessageSender, view: Arc<Mutex<View>>) -> ProjectTree {
        ProjectTree {
            messagesender,
            view,
            projectnode_root: ProjectNode::new(String::from("Project"), String::from("./")),
            display_lines: Vec::new(),
        }
    }
}

impl UIComponent for ProjectTree {
    fn display(&mut self) {
        let display_lines = self.projectnode_root.get_display_lines(0);
        let view = self.view.lock().unwrap();
        view.set_content(display_lines);
    }
}
