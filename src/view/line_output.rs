use std::{collections::VecDeque, cell::{RefCell, Ref}, rc::Rc};

use chiropterm::{Brush, FSem, colors::{Black, White}, CellRect, CellPoint, CellSize, Stamp, Brushable};
use chiroptui::{WidgetMenu, CanvasState};
use euclid::{rect, vec2};

pub struct LineOutput {
    output: Rc<RefCell<LineOutputImpl>>
}

impl LineOutput {
    pub fn new() -> Self {
        LineOutput { output: Rc::new(RefCell::new(LineOutputImpl::new())) }
    }

    pub fn share(&self) -> Self {
        return LineOutput { output: self.output.clone() }
    }

    pub(crate) fn draw(&self, brush: Brush, menu: WidgetMenu<CanvasState>) {
        self.output.borrow_mut().draw(brush, menu);
    }

    pub(crate) fn add_line(&self, indent: isize, postdent: isize, text: String) {
        self.output.borrow_mut().add_line(indent, postdent, text)
    }
}

struct LineOutputImpl {
    lines: VecDeque<Line>,
}

impl LineOutputImpl {
    fn new() -> Self {
        Self { lines: VecDeque::new(), } 
    }


    pub fn add_line(&mut self, indent: isize, postdent: isize, text: String) {
        self.lines.push_back(Line::new(indent, postdent, text));
        while self.lines.len() > 80 {
            self.lines.pop_front();
        }
    }

    pub fn draw(&mut self, brush: Brush, _menu: WidgetMenu<CanvasState>) {
        brush.fill(FSem::new().color((Black, White)));

        let width = brush.rect().width();
        let mut y = brush.rect().max_y();

        for l in self.lines.iter().rev() {
            let stamp = l.stamp_for(width - l.indent - l.postdent);
            let height = stamp.1.rect().height().max(1);
            y -= height;
            stamp.1.draw(brush.region(
                CellRect::new(
                    CellPoint::new(brush.rect().min_x() + l.indent, y),
                    CellSize::new(brush.rect().width() - l.indent - l.postdent, height),
                )
            ));

            if y < brush.rect().min_y() { break; }
        }
    }
}

struct Line {
    pub indent: isize,
    pub postdent: isize,
    pub text: String,
    pub stamp: RefCell<(isize, Stamp)>,
}

impl Line {
    fn new(indent: isize, postdent: isize, text: String) -> Self {
        return Self {
            indent, postdent,
            text,
            stamp: RefCell::new((-1, Stamp::new())),
        }
    }
    fn stamp_for(&self, width: isize) -> Ref<'_, (isize, Stamp)> {
        const HANGING_INDENT: isize = 0; // Nyeo note: this system totally breaks word wrap rn
        {
            let s = self.stamp.borrow();
            if s.0 == width {
                return s
            }
        }

        let stamp = Stamp::new();
        let brush = stamp
            .brush_at(rect(0, 0, width, isize::MAX))
            .offset_rect(vec2(HANGING_INDENT, 0));
        brush.at(CellPoint::new(-HANGING_INDENT, 0)).putfs(&self.text);
        self.stamp.replace((width, stamp));
        self.stamp.borrow()
    }
}