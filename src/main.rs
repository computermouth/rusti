
#[derive(Copy, Clone)]
struct Polygon {
    x: &'static [i16],
    y: &'static [i16],
    scaled_x: &'static [i16],
    scaled_y: &'static [i16],
}

#[derive(Copy, Clone)]
struct Frame {
    polygons: &'static [Polygon],
}

#[derive(Copy, Clone)]
struct Animation {
    frames: &'static [Frame]
}

#[derive(Copy, Clone)]
struct SpriteData {
    animations:  &'static [Animation],
}

static SPRITE: SpriteData = SpriteData {
    animations: &[
        Animation {
            frames: &[
                Frame {
                    polygons: &[
                        Polygon {
                            x: &[1, 2, 3],
                            y: &[4, 5, 6],
                            scaled_x: &[2, 4, 6],
                            scaled_y: &[4, 5, 6],
                        },
                    ],
                },
            ],
        },
        Animation {
            frames: &[
                Frame {
                    polygons: &[
                        Polygon {
                            x: &[1, 2, 3],
                            y: &[4, 5, 6],
                            scaled_x: &[1, 2, 3],
                            scaled_y: &[4, 5, 6],
                        },
                        Polygon {
                            x: &[1, 2, 3],
                            y: &[4, 5, 6],
                            scaled_x: &[1, 2, 3],
                            scaled_y: &[4, 5, 6],
                        },
                    ],
                },
                Frame {
                    polygons: &[
                        Polygon {
                            x: &[1, 2, 3],
                            y: &[4, 5, 6],
                            scaled_x: &[1, 2, 3],
                            scaled_y: &[4, 5, 6],
                        },
                        Polygon {
                            x: &[1, 2, 3],
                            y: &[4, 5, 6],
                            scaled_x: &[1, 2, 3],
                            scaled_y: &[4, 5, 6],
                        },
                    ],
                },
            ],
        },
    ],
};

fn main() {
	
	let sprite: SpriteData = SPRITE;
	
    println!("{:p}", &sprite);
    println!("{:p}", &sprite.animations);
    println!("{:p}", &sprite.animations[0].frames);
    println!("{:p}", &sprite.animations[0].frames[0].polygons);
    println!("{:p}", &sprite.animations[0].frames[0].polygons[0].x);
    println!("{:p}", &sprite.animations[0].frames[0].polygons[0].y);
    println!("{:p}", &sprite.animations[0].frames[0].polygons[0].scaled_x);
    println!("{:p}", &sprite.animations[0].frames[0].polygons[0].scaled_y);
    println!("{:p}", &sprite.animations[1].frames);
    println!("{:p}", &sprite.animations[1].frames[0].polygons);
    println!("{:p}", &sprite.animations[1].frames[0].polygons[0].x);
    println!("{:p}", &sprite.animations[1].frames[0].polygons[0].y);
    println!("{:p}", &sprite.animations[1].frames[0].polygons[0].scaled_x);
    println!("{:p}", &sprite.animations[1].frames[0].polygons[0].scaled_y);
    
    
    println!("scaled_x {:?}", sprite.animations[0].frames[0].polygons[0].scaled_x);
    for i in 0..sprite.animations[0].frames[0].polygons[0].scaled_x.len() {
		let fi: f32 = sprite.animations[0].frames[0].polygons[0].scaled_x[i] as f32 * 2.5;
		sprite.animations[0].frames[0].polygons[0].scaled_x[i] = fi as i16;
	}
    println!("scaled_x {:?}", sprite.animations[0].frames[0].polygons[0].scaled_x);
}
















