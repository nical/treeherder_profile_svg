use serde::Deserialize;
use svg_fmt::*;

#[derive(Deserialize)]
struct Profile {
    samples: Vec<Sample>,
    cpu_times_fields: Vec<String>,
    io_fields: Vec<String>,
    //start: f64,
    //end: f64,
    //duration: f64,
    //overall: Overall,
}

#[derive(Deserialize)]
struct Sample {
    cpu_percent_cores: Vec<f64>,
    //cpu_precent_mean: f64,
    //cpu_times: Vec<Vec<f64>>,
    cpu_times_sum: Vec<f64>,
    cpu_times_total: f64,
    //start: f64,
    //end: f64,
    io: Vec<f64>,
    swap: Vec<f64>,
    virt: Vec<f64>,
}

const PADDING: f32 = 10.0;
const SAMPLE_W: f32 = 2.0;
const TRACK_H: f32 = 40.0;
const TRACK_SEP: f32 = 40.0;
const TEXT_SIZE: f32 = 30.0;

fn main() -> std::io::Result<()> {
    let mut args = std::env::args();
    let _ = args.next();
    let input_path = args.next().expect("Must pass a file as input");

    let file = std::fs::File::open(&input_path)?;
    let profile: Profile = serde_json::from_reader(file)?;

    let white = Color { r: 255, g: 255, b: 255 };

    let num_samples = profile.samples.len();
    let s = &profile.samples[0];

    let cpu_percent_cores_h = s.cpu_percent_cores.len() as f32 * TRACK_H;
    let cpu_percent_cores_total_h = cpu_percent_cores_h / 4.0;
    let cpu_times_h = s.cpu_times_sum.len() as f32 * TRACK_H;
    let io_h = s.io.len() as f32 * TRACK_H;
    let swap_h = s.swap.len() as f32 * TRACK_H;
    let virt_h = s.virt.len() as f32 * TRACK_H;
    let border_radius = 5.0;

    let h = PADDING
        + TRACK_SEP
        + TRACK_H + TRACK_SEP
        + cpu_percent_cores_h + TRACK_SEP
        + cpu_percent_cores_total_h + TRACK_SEP
        + cpu_times_h
        + io_h + TRACK_SEP
        + swap_h + TRACK_SEP
        + virt_h + TRACK_SEP
        + PADDING;

    let track_w = num_samples as f32 * SAMPLE_W;
    let w = track_w + PADDING * 2.0;

    let bg_style = Style {
        fill: Fill::Color(Color { r: 200, g: 200, b: 200 }),
        stroke: Stroke::None,
        opacity: 1.0,
        stroke_opacity: 1.0
    };

    let graph_style = Style {
        fill: Fill::Color(Color { r: 10, g: 100, b: 255 }),
        stroke: Stroke::None,
        opacity: 1.0,
        stroke_opacity: 1.0
    };
    
    println!("{}", BeginSvg { w, h });

    println!("    {}", Rectangle {
        x: -1.0, y: -1.0, w: w + 2.0, h: h + 2.0,
        style: Style {
            fill: Fill::Color(Color { r: 50, g: 50, b: 50}),
            stroke: Stroke::None,
            opacity: 1.0,
            stroke_opacity: 1.0,
        },
        border_radius: 0.0
    });

    let x = PADDING;
    let mut y = PADDING + TRACK_SEP;

    println!("    {}", Rectangle { x, y, w: track_w, h: TRACK_H / 2.0, style: bg_style, border_radius });
    println!("    {}", label(x, y, "Time (min)"));

    let num_min = num_samples / 60;
    for i in 1..(num_min + 1) {
        let lx = x + i as f32 * 60.0 * SAMPLE_W;
        println!("    {}", LineSegment { x1: lx, y1: y, x2: lx, y2: y + TRACK_H / 2.0, color: Color { r: 50, g: 50, b: 50 }, width: 2.0 });
    }

    y += TRACK_H / 2.0 + TRACK_SEP;

    println!("    {}", Rectangle { x, y, w: track_w, h: cpu_percent_cores_h, style: bg_style, border_radius });
    println!("    {}", label(x, y, "CPU % cores"));

    for i in 0..s.cpu_percent_cores.len() {
        let py = y + i as f32 * TRACK_H;

        println!("    {}", make_graph(x, py, track_w, TRACK_H,
            &mut|s| profile.samples[s].cpu_percent_cores[i] as f32,
            num_samples,
            100.0,
            graph_style
        ));
    }

    y += cpu_percent_cores_h + TRACK_SEP;

    println!("    {}", Rectangle { x, y, w: track_w, h: cpu_percent_cores_total_h, style: bg_style, border_radius });
    println!("    {}", label(x, y, "CPU % total"));

    println!("    {}", make_graph(x, y, track_w, cpu_percent_cores_total_h,
        &mut|si| profile.samples[si].cpu_percent_cores.iter().sum::<f64>() as f32,
        num_samples,
        100.0 * s.cpu_percent_cores.len() as f32,
        graph_style
    ));

    for i in 1..s.cpu_percent_cores.len() {
        let y = y + i as f32 * TRACK_H / 4.0;
        println!("    {}", LineSegment { x1: x, y1: y, x2: x+track_w, y2: y, width: 0.5, color: white, });
    }

    y += cpu_percent_cores_total_h + TRACK_SEP;


    println!("    {}", Rectangle { x, y, w: track_w, h: cpu_times_h, style: bg_style, border_radius });
    println!("    {}", label(x, y, "CPU time types"));

    for i in 0..s.cpu_times_sum.len() {
        let py = y + i as f32 * TRACK_H;

        println!("    {}", make_graph(x, py, track_w, TRACK_H,
            &mut|s| (profile.samples[s].cpu_times_sum[i] / profile.samples[s].cpu_times_total) as f32,
            num_samples,
            1.0,
            graph_style
        ));

        track_label(x, py, &profile.cpu_times_fields[i]);
    }

    y += cpu_times_h + TRACK_SEP;

    println!("    {}", Rectangle { x, y, w: track_w, h: io_h, style: bg_style, border_radius });
    println!("    {}", label(x, y, "io"));

    for i in 0..s.io.len() {
        let py = y + i as f32 * TRACK_H;

        println!("    {}", make_graph(x, py, track_w, TRACK_H,
            &mut|s| profile.samples[s].io[i] as f32,
            num_samples,
            1_000_000.0,
            graph_style
        ));

        track_label(x, py, &profile.io_fields[i]);
    }

    y += io_h + TRACK_SEP;

    println!("    {}", Rectangle { x, y, w: track_w, h: swap_h, style: bg_style, border_radius });
    println!("    {}", label(x, y, "swap"));

    for i in 0..s.swap.len() {
        let py = y + i as f32 * TRACK_H;

        println!("    {}", make_graph(x, py, track_w, TRACK_H,
            &mut|s| profile.samples[s].swap[i] as f32,
            num_samples,
            10_000_000_000.0,
            graph_style
        ));
    }

    y += swap_h + TRACK_SEP;

    println!("    {}", Rectangle { x, y, w: track_w, h: virt_h, style: bg_style, border_radius });
    println!("    {}", label(x, y, "virt"));

    for i in 0..s.virt.len() {
        let py = y + i as f32 * TRACK_H;

        println!("    {}", make_graph(x, py, track_w, TRACK_H,
            &mut|s| profile.samples[s].virt[i] as f32,
            num_samples,
            100_000_000_000.0,
            graph_style
        ));
    }


    println!("{}", EndSvg);

    Ok(())
}

fn label(x: f32, y: f32, text: &str) -> Text {
    Text {
        x,
        y: y - 5.0,
        text: text.to_string(),
        color: Color { r: 255, g: 255, b: 255 },
        align: Align::Left,
        size: TEXT_SIZE,
    }
}

fn track_label(x: f32, y: f32, text: &str) {
    let text_size = TEXT_SIZE / 1.5;

    let label = Text {
        x: x + 3.0,
        y: y + text_size - 1.0,
        text: text.to_string(),
        color: Color { r: 255, g: 255, b: 255 },
        align: Align::Left,
        size: text_size,
    };

    println!("    {}", Rectangle {
        x, y: y + 3.0, w: 110.0, h: text_size,
        style: Style {
            fill: Fill::Color(Color { r: 0, g: 0, b: 0}),
            stroke: Stroke::None,
            opacity: 0.4,
            stroke_opacity: 1.0,
        },
        border_radius: 4.0
    });

    println!("    {}", label);
}

fn make_graph(
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    cb: &mut dyn FnMut(usize) -> f32,
    count: usize,
    y_max: f32,
    style: Style
) -> Path {
    let sample_w = width / count as f32;    
    let mut px = x;

    let mut p = Path { ops: Vec::new(), style };
    p = p.move_to(x, y + height);
    for i in 0..count {
        let norm_val = cb(i).min(y_max) / y_max;
        p = p.line_to(px, y + height - norm_val * height);
        px += sample_w;
    }
    p = p.line_to(px, y + height);

    p
}

