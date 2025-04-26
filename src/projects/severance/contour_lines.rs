use super::colors;
use delaunator::{triangulate, Point};
use nannou::prelude::*;
use petgraph::{
    graph::{NodeIndex, UnGraph},
    visit::EdgeRef,
};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, Debug, PartialEq)]
struct ContourPoint {
    position: Point2,
    elevation: f32,
}

pub struct Model {
    graph: UnGraph<ContourPoint, ()>,
}

impl Model {
    pub fn new(app: &App) -> Self {
        app.new_window()
            .size(640, 800)
            .view(view)
            .key_pressed(key_pressed)
            .build()
            .unwrap();
        let points = Model::new_points(&app.window_rect());

        Model {
            graph: delaunay_triangulation(points),
        }
    }

    fn new_points(window: &Rect) -> Vec<ContourPoint> {
        (0..10)
            .map(|_| ContourPoint {
                position: pt2(
                    random_range(window.x.start, window.x.end),
                    random_range(window.y.start, window.y.end),
                ),
                elevation: random::<f32>() * 50.,
            })
            .collect()
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(colors::DARKBLUE);

    for edge in model.graph.edge_references() {
        let start = model.graph[edge.source()];
        let end = model.graph[edge.target()];
        draw.line()
            .start(start.position)
            .end(end.position)
            .weight(2.0)
            .color(colors::LIGHTBLUE);
    }
    let points = elevation_thing(&model.graph, 20.);
    for point in &points {
        draw.ellipse().xy(*point).radius(5.).color(colors::RED);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn delaunay_triangulation(points: Vec<ContourPoint>) -> UnGraph<ContourPoint, ()> {
    let delaunator_points: Vec<Point> = points
        .iter()
        .map(|point| Point {
            x: point.position.x as f64,
            y: point.position.y as f64,
        })
        .collect();
    let triangulation = triangulate(&delaunator_points);

    // Build the graph
    let mut graph = UnGraph::<ContourPoint, ()>::new_undirected();
    let node_indices: Vec<NodeIndex> = (0..points.len())
        .map(|i| graph.add_node(points[i]))
        .collect();

    // To avoid duplicate edges, use a set
    let mut edge_set = HashSet::new();
    for triangle in triangulation.triangles.chunks(3) {
        let a = triangle[0];
        let b = triangle[1];
        let c = triangle[2];
        // Edges: (a,b), (b,c), (c,a)
        for &(u, v) in &[(a, b), (b, c), (c, a)] {
            let edge = if u < v { (u, v) } else { (v, u) };
            if edge_set.insert(edge) {
                graph.add_edge(node_indices[u], node_indices[v], ());
            }
        }
    }

    graph
}

fn elevation_thing(graph: &UnGraph<ContourPoint, ()>, elevation: f32) -> Vec<Point2> {
    let mut points = vec![];
    for edge in graph.edge_references() {
        let start = graph[edge.source()];
        let end = graph[edge.target()];
        if !is_between(elevation, start.elevation, end.elevation) {
            continue;
        };
        if start.elevation < end.elevation {
            let thing = map_range(elevation, start.elevation, end.elevation, 0., 1.);
            points.push(start.position + thing * (end.position - start.position));
        } else {
            let thing = map_range(elevation, end.elevation, start.elevation, 0., 1.);
            points.push(end.position + thing * (start.position - end.position));
        }
    }
    points
}

/** Handles both increasing and decreasing intervals, and excludes NaN */
fn is_between(x: f32, a: f32, b: f32) -> bool {
    if a < b {
        x >= a && x <= b && !x.is_nan() && !a.is_nan() && !b.is_nan()
    } else {
        x >= b && x <= a && !x.is_nan() && !a.is_nan() && !b.is_nan()
    }
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Space => {
            let points = Model::new_points(&app.window_rect());
            model.graph = delaunay_triangulation(points);
        }
        _ => return,
    }
}

fn thing2(
    graph: &UnGraph<ContourPoint, ()>,
    convex_hull: Vec<usize>,
    elevation: f32,
) -> UnGraph<Point2, ()> {
    // Assume NewPoint is your struct for the new points
    let mut elevation_points_graph = UnGraph::<Point2, ()>::new_undirected();
    let elevation_points = elevation_thing(&graph, elevation);

    // Map from your point data to node indices
    let mut point_to_index = HashMap::new();

    // Add nodes for each new point
    for pt in &elevation_points {
        let idx = graph.add_node(pt.clone());
        point_to_index.insert(pt.id, idx);
    }

    // For each triangle, connect new points on its edges
    for triangle in &triangles {
        let new_points_in_triangle: Vec<_> = triangle
            .edges()
            .filter_map(|edge| new_point_on_edge(edge))
            .collect();
        for i in 0..new_points_in_triangle.len() {
            for j in (i + 1)..new_points_in_triangle.len() {
                graph.add_edge(
                    point_to_index[&new_points_in_triangle[i].id],
                    point_to_index[&new_points_in_triangle[j].id],
                    (),
                );
            }
        }
    }

    for i in 0..convex_hull.len() {
        for j in (i + 1)..convex_hull.len() {
            graph.add_edge(
                point_to_index[&convex_hull[i].id],
                point_to_index[&convex_hull[j].id],
                (),
            );
        }
    }

    elevation_points_graph
}
