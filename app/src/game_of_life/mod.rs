use std::cmp::PartialEq;
use vertigo::{
    Css,
    DomDriver,
    VDomNode,
    computed::{
        Computed,
        Dependencies,
        Value
    },
    node_attr,
    utils::DropResource
};

mod next_generation;

fn create_matrix_row(root: &Dependencies, x_count: u16) -> Vec<Value<bool>> {
    let mut row = Vec::new();

    for _ in 0..x_count {
        row.push(root.new_value(false));
    }

    row
}

fn create_matrix(root: &Dependencies, x_count: u16, y_count: u16) -> Vec<Vec<Value<bool>>> {
    let mut matrix = Vec::new();

    for _ in 0..y_count {
        matrix.push(create_matrix_row(root, x_count));
    }

    matrix
}

#[derive(PartialEq)]
pub struct State {
    pub dom_driver: DomDriver,
    pub root: Dependencies,
    pub x_count: Value<u16>,
    pub y_count: Value<u16>,
    pub matrix: Computed<Vec<Vec<Value<bool>>>>,
    pub timer_enable: Value<bool>,
    pub year: Value<u32>,
}

impl State {
    pub fn new(root: &Dependencies, dom_driver: &DomDriver) -> Computed<State> {
        let x_count = 120;
        let y_count = 70;

        root.new_computed_from(State {
            dom_driver: dom_driver.clone(),
            root: root.clone(),
            x_count: root.new_value(x_count),
            y_count: root.new_value(y_count),
            matrix: root.new_computed_from(create_matrix(root, x_count, y_count)),
            timer_enable: root.new_value(false),
            year: root.new_value(0),
        })
    }

    pub fn start_timer(&self) -> DropResource {
        let year = self.year.clone();
        let timer_enable = self.timer_enable.clone();

        let root = self.root.clone();
        let x_count = self.x_count.clone();
        let y_count = self.y_count.clone();
        let matrix = self.matrix.clone();

        self.dom_driver.set_interval(100, move || {

            let timer_enable = timer_enable.get_value();

            if *timer_enable {
                let current = year.get_value();
                year.set_value(*current + 1);

                let x_count = x_count.get_value();
                let y_count = y_count.get_value();
                let matrix = matrix.get_value();

                next_generation::next_generation(&root, *x_count, *y_count, &*matrix)
            }
        })
    }
}

fn css_wrapper() -> Css {
    Css::one("
        border: 1px solid black;
        padding: 10px;
        margin: 10px;
        background-color: #e0e0e0;
    ")
}

fn css_row() -> Css {
    Css::one("
        display: flex;
        flex-direction: row;
        height: 10px
    ")
}

fn css_cell(is_active: bool) -> Css {
    let mut css = Css::one("
        width: 10px;
        height: 10px;
        cursor: pointer;
    ");

    if is_active {
        css.str("background-color: black");
    } else {
        css.str("background-color: white");
    }

    css
}

fn css_button() -> Css {
    Css::one("
        cursor: pointer;
    ")
}

fn render_header(state: &Computed<State>) -> VDomNode {
    use node_attr::{buildNode, css, node, text, onClick};

    let state = state.get_value();
    let year = state.year.get_value();
    let timer_enable = state.timer_enable.get_value();

    let button = if *timer_enable {
        node("button", vec!(
            css(css_button()),
            text("Stop"),
            onClick({
                let timer_enable = state.timer_enable.clone();
                move || {
                    timer_enable.set_value(false);
                    log::info!("stop ...");
                }
            })
        ))
    } else {
        node("button", vec!(
            css(css_button()),
            text("Start"),
            onClick({
                let timer_enable = state.timer_enable.clone();
                move || {
                    timer_enable.set_value(true);
                    log::info!("start ...");
                }
            })
        ))
    };

    buildNode("div", vec!(
        node("div", vec!(
            text("Game of life")
        )),
        node("div", vec!(
            text(format!("year = {}", year))
        )),
        button
    ))
}

pub fn render(state: &Computed<State>) -> VDomNode {
    use node_attr::{buildNode, css, component};

    let value = state.get_value().matrix.get_value();
    let value_inner = &*value;

    buildNode("div", vec!(
        css(css_wrapper()),
        component(state.clone(), render_header),
        render_matrix(value_inner)
    ))
}

fn render_matrix(matrix: &Vec<Vec<Value<bool>>>) -> node_attr::NodeAttr {
    use node_attr::{node};

    let mut out: Vec<node_attr::NodeAttr> = Vec::new();

    for item in matrix.iter() {
        out.push(render_row(item));
    }

    node("div", out)
}

fn render_row(matrix: &Vec<Value<bool>>) -> node_attr::NodeAttr {
    use node_attr::{node, css, component_value};

    let mut out: Vec<node_attr::NodeAttr> = Vec::new();

    out.push(css(css_row()));

    for item in matrix.iter() {
        out.push(component_value(item.clone(), render_cell));
    }

    node("div", out)
}

fn render_cell(cell: &Value<bool>) -> VDomNode {
    use node_attr::{buildNode, css, onClick};

    let is_active = cell.get_value();

    let on_click = {
        let cell = cell.clone();
        let is_active = *is_active;

        move || {
            cell.set_value(!is_active);
        }
    };

    buildNode("div", vec!(
        onClick(on_click),
        css(css_cell(*is_active)),
    ))
}
