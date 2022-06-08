use std::cmp::{max, min};
use std::collections::BTreeMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use plotters::prelude::*;

use serde::{Deserialize, Serialize};

use crate::economy::entity::Consumer;
use crate::economy::entity::Producer;
use crate::economy::function::FunctionAbstract;
use crate::economy::geography::City;
use crate::economy::geography::CityId;
use crate::economy::geography::Connection;
use crate::economy::geography::Geography;
use crate::economy::market::Market;
use crate::economy::types::InnerValue;

pub type ArgT = crate::economy::types::Price;
pub type ValueT = crate::economy::types::Volume;

#[derive(Serialize, Deserialize, Debug)]
pub struct SimulationBuilder {
    turns: usize,
    cities: Vec<City>,
    connections: Vec<Connection>,
    initial_prices: Vec<(CityId, ArgT)>,
    producers: Vec<Producer>,
    consumers: Vec<Consumer>,
}

#[derive(Debug)]
pub struct Simulation {
    turns: usize,
    pub market: Market,
    producers: Vec<Producer>,
    consumers: Vec<Consumer>,
}

impl Simulation {
    fn new(turns: usize, geography: Geography, prices: BTreeMap<CityId, ArgT>) -> Simulation {
        Simulation {
            turns,
            market: Market::new(geography, prices),
            producers: vec![],
            consumers: vec![],
        }
    }

    pub fn read_from_file<P: AsRef<Path>>(path: P) -> Result<Simulation, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let simulation_builder: SimulationBuilder = serde_json::from_reader(reader)?;

        let mut geography = Geography::new();
        for city in simulation_builder.cities {
            geography.add_city(city);
        }
        for connection in simulation_builder.connections {
            geography.add_connection(connection);
        }

        let mut simulation = Simulation::new(
            simulation_builder.turns,
            geography,
            simulation_builder.initial_prices.into_iter().collect(),
        );
        for producer in simulation_builder.producers {
            simulation.add_producer(producer);
        }
        for consumer in simulation_builder.consumers {
            simulation.add_consumer(consumer);
        }

        Ok(simulation)
    }

    fn add_producer(&mut self, producer: Producer) {
        self.market.add_producer(&producer);
        self.producers.push(producer)
    }

    fn add_consumer(&mut self, consumer: Consumer) {
        self.market.add_consumer(&consumer);
        self.consumers.push(consumer)
    }

    fn simulate_turn(&mut self) {
        self.market.simulate(1);
        for prod in &mut self.producers {
            prod.update(&mut self.market)
        }
        for cons in &mut self.consumers {
            cons.update(&mut self.market)
        }
    }

    pub fn run(&mut self) {
        for _ in 0..self.turns {
            self.simulate_turn();
        }
    }

    pub fn plot(&mut self, output_file: &str) -> Result<(), Box<dyn Error>> {
        /* general settings */
        const HEAD_SIZE_Y: u32 = 128;
        const PLOT_SIZE_X: u32 = 1024;
        const PLOT_SIZE_Y: u32 = 768;
        const MARGIN: u32 = 20;
        const LABEL_AREA_SIZE: u32 = 50;
        const TITLE_FONT_SIZE: u32 = 60;
        const CAPTION_FONT_SIZE: u32 = 40;
        const MAX_X_LABELS_CNT: usize = 8;
        const MAX_Y_LABELS_CNT: usize = 6;
        const SERIES_STEPS: InnerValue = 128.0;
        const DOTTED_STEPS_VERTICAL: InnerValue = 84.0;
        const DOTTED_STEPS_HORIZONTAL: InnerValue = 112.0;
        const SERIES_WIDTH: u32 = 3;
        const EXCHANGE_WIDTH: u32 = 5;
        const LEGEND_WIDTH: u32 = 2;
        const GREY: RGBColor = RGBColor(64, 64, 64);
        const GREEN_DARK: RGBColor = RGBColor(0, 176, 0);

        /* number of cities to plot for */
        let plot_count: u32 = self.market.geography().cities().len() as u32;

        /* root plotting area */
        let root_area = BitMapBackend::new(
            output_file,
            (PLOT_SIZE_X, HEAD_SIZE_Y + PLOT_SIZE_Y * plot_count),
        )
        .into_drawing_area();
        root_area.fill(&WHITE)?;
        let mut root_area =
            root_area.titled("Supplies & Demands", ("sans-serif", TITLE_FONT_SIZE))?;

        /* main plotting loop */
        for city in self.market.geography().cities() {
            let city_data = self.market.cities().get(&city.id).unwrap();

            /* city specific values */
            let min_x: ArgT = min(
                city_data.supply().function().min_arg(),
                city_data.demand().function().min_arg(),
            );
            let max_x: ArgT = max(
                city_data.supply().function().max_arg(),
                city_data.demand().function().max_arg(),
            );
            let min_y: ValueT = min(
                city_data.supply().function().min_value(),
                city_data.demand().function().min_value(),
            );
            let max_y: ValueT = max(
                city_data.supply().function().max_value(),
                city_data.demand().function().max_value(),
            ) * 1.1;
            let exchange_min: ValueT = min(
                city_data.supply_volume().unwrap(),
                city_data.demand_volume().unwrap(),
            );
            let exchange_max: ValueT = max(
                city_data.supply_volume().unwrap(),
                city_data.demand_volume().unwrap(),
            );

            /* steps for specific plots */
            let series_step: ArgT = (max_x - min_x) / SERIES_STEPS;
            let exchange_step: ValueT = (exchange_max - exchange_min) / SERIES_STEPS;
            let dotted_step_horizontal: ArgT = (max_x - min_x) / DOTTED_STEPS_HORIZONTAL;
            let dotted_step_vertical: ValueT = (max_y - min_y) / DOTTED_STEPS_VERTICAL;

            /* acquire plotting area for current city */
            let (current_area, remaining_area) = root_area.split_vertically(PLOT_SIZE_Y);
            root_area = remaining_area;

            /* ranges for x_axis functions and exchange */
            let x_axis = (min_x.float()..max_x.float()).step(series_step.float());
            let exchange_line_vertical =
                (exchange_min.float()..exchange_max.float()).step(exchange_step.float());

            /* plot initialization */
            let mut chart_builder = ChartBuilder::on(&current_area)
                .margin(MARGIN)
                .set_label_area_size(LabelAreaPosition::Left, LABEL_AREA_SIZE)
                .set_label_area_size(LabelAreaPosition::Right, LABEL_AREA_SIZE)
                .set_label_area_size(LabelAreaPosition::Bottom, LABEL_AREA_SIZE)
                .caption(city.name.clone(), ("sans-serif", CAPTION_FONT_SIZE))
                .build_cartesian_2d(min_x.float()..max_x.float(), min_y.float()..max_y.float())?;

            /* plot configuration */
            chart_builder
                .configure_mesh()
                .x_desc("Price / Unit")
                .y_desc("Units")
                .x_labels(MAX_X_LABELS_CNT)
                .y_labels(MAX_Y_LABELS_CNT)
                .x_label_formatter(&|v| format!("{:.2}", v))
                .y_label_formatter(&|v| format!("{:.2}", v))
                .draw()?;

            /* marking the initial value of x_axis */
            chart_builder.draw_series(PointSeries::of_element(
                vec![(min_x.float(), min_y.float())],
                0,
                ShapeStyle::from(&BLACK).filled(),
                &|coord, size, style| {
                    EmptyElement::at(coord)
                        + Circle::new((0, 0), size, style)
                        + Text::new(format!("{:.2}", min_x.float()), (0, 10), ("sans-serif", 12))
                },
            ))?;

            /* marking the initial value of y_axis */
            chart_builder.draw_series(PointSeries::of_element(
                vec![(min_x.float(), min_y.float())],
                0,
                ShapeStyle::from(&BLACK).filled(),
                &|coord, size, style| {
                    EmptyElement::at(coord)
                        + Circle::new((0, 0), size, style)
                        + Text::new(
                            format!("{:.2}", min_y.float()),
                            (-30, -8),
                            ("sans-serif", 12),
                        )
                },
            ))?;

            /* drawing the supply function */
            chart_builder
                .draw_series(LineSeries::new(
                    x_axis
                        .values()
                        .map(|x| (x, city_data.supply().value(ArgT::new(x)).float())),
                    Into::<ShapeStyle>::into(&BLUE)
                        .filled()
                        .stroke_width(SERIES_WIDTH),
                ))?
                .label("Supply")
                .legend(|(x, y)| {
                    PathElement::new(
                        vec![(x, y), (x + 25, y)],
                        Into::<ShapeStyle>::into(&BLUE)
                            .filled()
                            .stroke_width(LEGEND_WIDTH),
                    )
                });

            /* drawing the demand function */
            chart_builder
                .draw_series(LineSeries::new(
                    x_axis
                        .values()
                        .map(|x| (x, city_data.demand().value(ArgT::new(x)).float())),
                    Into::<ShapeStyle>::into(&RED)
                        .filled()
                        .stroke_width(SERIES_WIDTH),
                ))?
                .label("Demand")
                .legend(|(x, y)| {
                    PathElement::new(
                        vec![(x, y), (x + 25, y)],
                        Into::<ShapeStyle>::into(&RED)
                            .filled()
                            .stroke_width(LEGEND_WIDTH),
                    )
                });

            /* drawing the exchange */
            chart_builder
                .draw_series(LineSeries::new(
                    exchange_line_vertical.values().map(|y| (min_x.float(), y)),
                    Into::<ShapeStyle>::into(&GREEN_DARK)
                        .filled()
                        .stroke_width(EXCHANGE_WIDTH),
                ))?
                .label("Exchange")
                .legend(|(x, y)| {
                    PathElement::new(
                        vec![(x, y), (x + 25, y)],
                        Into::<ShapeStyle>::into(&GREEN_DARK)
                            .filled()
                            .stroke_width(LEGEND_WIDTH),
                    )
                });

            /* drawing the chart legend */
            chart_builder
                .configure_series_labels()
                .border_style(&BLACK)
                .draw()?;

            /* three main interest points of the plot */
            let intersection: Option<(ArgT, ValueT)> = city_data
                .supply()
                .function()
                .intersect(city_data.demand().function());
            let local_supply: (ArgT, ValueT) = (
                city_data.price().unwrap(),
                city_data.supply_volume().unwrap(),
            );
            let local_demand: (ArgT, ValueT) = (
                city_data.price().unwrap(),
                city_data.demand_volume().unwrap(),
            );

            let mut interest_points: Vec<((ArgT, ValueT), String)> = vec![
                (local_supply, String::from("current supply")),
                (local_demand, String::from("current demand")),
            ];
            if let Some(..) = intersection {
                interest_points.push((intersection.unwrap(), String::from("no exchange")));
            }

            /* loop for marking the interest points on the plot */
            for (point, description) in interest_points {
                /* ranges for drawing dotted lines between points */
                let dotted_line_vertical =
                    (min_y.float()..point.1.float()).step(dotted_step_vertical.float());
                let dotted_line_horizontal =
                    (min_x.float()..point.0.float()).step(dotted_step_horizontal.float());

                /* point on the plot */
                chart_builder.draw_series(PointSeries::of_element(
                    vec![(point.0.float(), point.1.float())],
                    5,
                    ShapeStyle::from(&GREY).filled(),
                    &|coord, size, style| {
                        EmptyElement::at(coord)
                            + Circle::new((0, 0), size, style)
                            + Text::new(description.clone(), (5, -18), ("sans-serif", 20))
                    },
                ))?;

                /* corresponding point on the x_axis */
                chart_builder.draw_series(PointSeries::of_element(
                    vec![(point.0.float(), min_y.float())],
                    2,
                    ShapeStyle::from(&GREY).filled(),
                    &|coord, size, style| {
                        EmptyElement::at(coord)
                            + Circle::new((0, 0), size, style)
                            + Text::new(
                                format!("{:.2}", point.0.float()),
                                (5, -16),
                                ("sans-serif", 18),
                            )
                    },
                ))?;

                /* dotted line connecting plot point and x_axis point */
                chart_builder.draw_series(PointSeries::of_element(
                    dotted_line_vertical.values().map(|y| (point.0.float(), y)),
                    1,
                    ShapeStyle::from(&GREY).filled(),
                    &|coord, size, style| {
                        EmptyElement::at(coord) + Circle::new((0, 0), size, style)
                    },
                ))?;

                if description != "no exchange" {
                    /* corresponding point on the y_axis */
                    chart_builder.draw_series(PointSeries::of_element(
                        vec![(min_x.float(), point.1.float())],
                        2,
                        ShapeStyle::from(&GREY).filled(),
                        &|coord, size, style| {
                            EmptyElement::at(coord)
                                + Circle::new((0, 0), size, style)
                                + Text::new(
                                    format!("{:.2}", point.1.float()),
                                    (5, -18),
                                    ("sans-serif", 18),
                                )
                        },
                    ))?;

                    /* dotted line connecting plot point and y_axis point */
                    chart_builder.draw_series(PointSeries::of_element(
                        dotted_line_horizontal
                            .values()
                            .map(|x| (x, point.1.float())),
                        1,
                        ShapeStyle::from(&GREY).filled(),
                        &|coord, size, style| {
                            EmptyElement::at(coord) + Circle::new((0, 0), size, style)
                        },
                    ))?;
                }
            }
        }

        /* final error check before return */
        root_area.present().expect(
            "Unable to save the results. Please make sure that the target
        directory exists under current directory and that target file has appropriate extension",
        );
        println!("Results have been saved to {}", output_file);
        Ok(())
    }
}
