use crate::core::{Message, MessageSender};
use crate::ui::{UIComponent, UISelector};
use crate::ui::{View};
use crate::terminal::DisplayLine;
use crate::core::Style;
use crate::core::HighlightEngine;

use std::fs;
use std::rc::Rc;
use std::cell::RefCell;

pub struct NodeLine {
    name: String,
    path: String,
    display_line: DisplayLine,
}

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
                    let name = format!("{}", entry.file_name().into_string().unwrap());
                    let path = format!("{}", entry.path().into_os_string().into_string().unwrap());
                    self.children.push(ProjectNode::new(name, path));
                }
            }
        }
    }

    pub fn get_display_lines(&self, parent_indent_level: usize) -> Vec<DisplayLine> {
        let title_content = " ".repeat(parent_indent_level*2) + "-" + &self.name.clone();
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
    view: Rc<RefCell<View>>,
    style: Style,
    projectnode_root: ProjectNode,
    display_lines: Vec<DisplayLine>,
}

impl ProjectTree {
    pub fn new(messagesender: MessageSender, view: Rc<RefCell<View>>, highlightengine: &HighlightEngine) -> ProjectTree {
        let mut projecttree = ProjectTree {
            messagesender,
            view,
            style: highlightengine.projecttree_style.clone(),
            projectnode_root: ProjectNode::new(String::from("Project"), String::from("./")),
            display_lines: Vec::new(),
        };
        projecttree.projectnode_root.build_children();
        projecttree
    }
}

impl UIComponent for ProjectTree {
    fn display(&mut self) {
        let display_lines = self.projectnode_root.get_display_lines(0);
        let view = self.view.borrow();
        view.set_content(display_lines, self.style);
    }
}
