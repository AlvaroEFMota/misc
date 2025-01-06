use eframe::egui;
use winit::platform::android::activity::AndroidApp;

#[unsafe(no_mangle)]
fn android_main(app: AndroidApp) -> eframe::Result {
    let options = eframe::NativeOptions {
        android_app: Some(app),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<MyApp>::default())
        }),
    )
}

struct MyApp {
    quadtree: QuadTree,
}

impl Default for MyApp {
    fn default() -> Self {
        let boundary = Rectangle {
            x: 200.,
            y: 400.,
            width: 170.0,
            height: 370.0,
        };

        Self {
            quadtree: QuadTree::new(boundary, 4),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let painter = ui.painter();
            let pointer = ui.input(|i| i.pointer.clone());

            let mouse_rect = if let Some(pos) = pointer.interact_pos() {
                let dist = 50.0;
                Rectangle::new(pos.x, pos.y, dist, dist)
            } else {
                Rectangle::new(0., 0., 0., 0.)
            };
            // Verifica se houve um clique
            if pointer.any_click() {
                if let Some(pos) = pointer.interact_pos() {
                    // Obtém a posição do cursor no clique
                    let point = Point::new(pos.x, pos.y);

                    if self.quadtree.insert(point) {
                        println!("inserted point: {:?}", pos);
                    }
                }
            }
            let points = self.quadtree.query(&mouse_rect);
            mouse_rect.draw(painter, egui::Color32::ORANGE);
            self.quadtree.draw(painter, egui::Color32::WHITE);
            for point in &points {
                point.draw(painter, 4.0, egui::Color32::ORANGE);
            }
        });
    }
}

#[derive(Clone, Debug)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    fn draw(&self, painter: &eframe::egui::Painter, radius: f32, color: eframe::egui::Color32) {
        painter.circle_filled(
            egui::Pos2::new(self.x, self.y), // Center
            radius,                          // Radius
            color,                           // Color
        );
    }
}

struct Rectangle {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl Rectangle {
    fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
    fn overlap(&self, point: &Point) -> bool {
        point.x >= self.x - self.width
            && point.x <= self.x + self.width
            && point.y >= self.y - self.height
            && point.y <= self.y + self.height
    }

    fn intersect(&self, other: &Rectangle) -> bool {
        let x_max = self.x + self.width;
        let x_min = self.x - self.width;
        let other_x_max = other.x + other.width;
        let other_x_min = other.x - other.width;
        let x_intersect = (x_min >= other_x_min && x_min <= other_x_max)
            || (other_x_min >= x_min && other_x_min <= x_max);

        let y_max = self.y + self.height;
        let y_min = self.y - self.height;
        let other_y_min = other.y - other.height;
        let other_y_max = other.y + other.height;
        let y_intersect = (y_min >= other_y_min && y_min <= other_y_max)
            || (other_y_min >= y_min && other_y_min <= y_max);

        return x_intersect && y_intersect;
    }

    fn draw(&self, painter: &eframe::egui::Painter, color: eframe::egui::Color32) {
        painter.rect_stroke(
            egui::Rect::from_min_max(
                egui::Pos2::new(self.x - self.width, self.y - self.height),
                egui::Pos2::new(self.x + self.width, self.y + self.height),
            ),
            0.0, // Rounding for corners
            egui::Stroke::new(1.0, color),
        );
    }
}

struct QuadTree {
    boundary: Rectangle,
    points: Vec<Point>,
    capacity: usize,
    subdivided: bool,
    northwest: Option<Box<QuadTree>>,
    northeast: Option<Box<QuadTree>>,
    southwest: Option<Box<QuadTree>>,
    southeast: Option<Box<QuadTree>>,
}

impl QuadTree {
    fn new(boundary: Rectangle, capacity: usize) -> Self {
        Self {
            boundary,
            points: Vec::new(),
            capacity,
            subdivided: false,
            northwest: None,
            northeast: None,
            southwest: None,
            southeast: None,
        }
    }

    fn query(&self, rect: &Rectangle) -> Vec<Point> {
        let mut points = Vec::new();
        self.query_process(rect, &mut points);
        points
    }

    fn query_process(&self, rect: &Rectangle, points: &mut Vec<Point>) {
        if !self.boundary.intersect(&rect) {
            return;
        }

        for point in &self.points {
            if rect.overlap(point) {
                points.push(point.clone());
            }
        }

        match (
            self.northwest.as_deref(),
            self.northeast.as_deref(),
            self.southwest.as_deref(),
            self.southeast.as_deref(),
        ) {
            (Some(nw), Some(ne), Some(sw), Some(se)) => {
                nw.query_process(rect, points);
                ne.query_process(rect, points);
                sw.query_process(rect, points);
                se.query_process(rect, points);
            }
            _ => {}
        }
    }

    fn draw(&self, painter: &eframe::egui::Painter, color: eframe::egui::Color32) {
        self.boundary.draw(painter, color.clone());
        for point in &self.points {
            point.draw(painter, 2.0, color.clone());
        }
        if self.subdivided {
            if let Some(qt) = &self.northwest {
                qt.draw(painter, color.clone());
            }
            if let Some(qt) = &self.northeast {
                qt.draw(painter, color.clone());
            }
            if let Some(qt) = &self.southwest {
                qt.draw(painter, color.clone());
            }
            if let Some(qt) = &self.southeast {
                qt.draw(painter, color.clone());
            }
        }
    }

    fn insert(&mut self, point: Point) -> bool {
        if !self.boundary.overlap(&point) {
            return false;
        }

        if self.points.len() < self.capacity {
            self.points.push(point);
            return true;
        } else {
            if !self.subdivided {
                let nw = Rectangle::new(
                    self.boundary.x - self.boundary.width / 2.0,
                    self.boundary.y + self.boundary.height / 2.0,
                    self.boundary.width / 2.0,
                    self.boundary.height / 2.0,
                );
                let ne = Rectangle::new(
                    self.boundary.x + self.boundary.width / 2.0,
                    self.boundary.y + self.boundary.height / 2.0,
                    self.boundary.width / 2.0,
                    self.boundary.height / 2.0,
                );
                let sw = Rectangle::new(
                    self.boundary.x - self.boundary.width / 2.0,
                    self.boundary.y - self.boundary.height / 2.0,
                    self.boundary.width / 2.0,
                    self.boundary.height / 2.0,
                );
                let se = Rectangle::new(
                    self.boundary.x + self.boundary.width / 2.0,
                    self.boundary.y - self.boundary.height / 2.0,
                    self.boundary.width / 2.0,
                    self.boundary.height / 2.0,
                );

                self.northwest = Some(Box::new(QuadTree::new(nw, 4)));
                self.northeast = Some(Box::new(QuadTree::new(ne, 4)));
                self.southwest = Some(Box::new(QuadTree::new(sw, 4)));
                self.southeast = Some(Box::new(QuadTree::new(se, 4)));
                self.subdivided = true;
            }

            match (
                self.northwest.as_deref_mut(),
                self.northeast.as_deref_mut(),
                self.southwest.as_deref_mut(),
                self.southeast.as_deref_mut(),
            ) {
                (Some(nw), Some(ne), Some(sw), Some(se)) => {
                    if nw.insert(point.clone()) {
                        return true;
                    } else if ne.insert(point.clone()) {
                        return true;
                    } else if sw.insert(point.clone()) {
                        return true;
                    } else if se.insert(point.clone()) {
                        return true;
                    }
                    false
                }
                _ => false,
            }
        }
    }
}
