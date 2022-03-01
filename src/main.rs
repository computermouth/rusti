
mod reftypes {
	//~ #[derive(Copy, Clone)]
	pub struct Polygon {
		pub x: &'static [i16],
		pub y: &'static [i16],
		pub scaled_x: &'static [i16],
		pub scaled_y: &'static [i16],
	}

	//~ #[derive(Copy, Clone)]
	pub struct Frame {
		pub polygons: &'static [Polygon],
	}

	//~ #[derive(Copy, Clone)]
	pub struct Animation {
		pub frames: &'static [Frame]
	}

	//~ #[derive(Copy, Clone)]
	pub struct Sprite {
		pub animations:  &'static [Animation],
	}
}

mod types {

	//~ #[derive(Copy, Clone)]
	#[derive(Debug)]
	pub struct Polygon {
		pub x: Vec<i16>,
		pub y: Vec<i16>,
		pub scaled_x: Vec<i16>,
		pub scaled_y: Vec<i16>,
	}

	//~ #[derive(Copy, Clone)]
	#[derive(Debug)]
	pub struct Frame {
		pub polygons: Vec<Polygon>
	}

	//~ #[derive(Copy, Clone)]
	#[derive(Debug)]
	pub struct Animation {
		pub frames: Vec<Frame>
	}

	//~ #[derive(Copy, Clone)]
	#[derive(Debug)]
	pub struct Sprite {
		pub animations:  Vec<Animation>,
	}
	
	//~ pub struct Reference {
		//~ arrays:   Vec<i16>,
		//~ frames:   Vec<isize>,
		//~ polygons: Vec<isize>,
		//~ vertices: Vec<isize>,
		//~ delays:   Vec<Vec<isize>>,
		//~ colors:   Vec<Vec<u8>>,
	//~ }
	
}

mod xpbit {
	
	//~ const localref: crate::types::Sprite = {};
	
	//~ const localref: types::Reference = {
		//~ animations: vec![
			//~ frames: vec![
				//~ polygons: vec![
					//~ types::Polygon{
						//~ x: vec![1, 2, 3]
						//~ y: vec![1, 2, 3]
						//~ scaled_x: vec![1, 2, 3]
						//~ scaled_y: vec![1, 2, 3]
					//~ },
				//~ ]
			//~ ]
		//~ ]
	//~ };
	
	//~ const localref: crate::types::Sprite = crate::types::Sprite {
		//~ animations: vec![
			
		//~ ]
	//~ };
	
	const LOCALREF: crate::reftypes::Sprite =  crate::reftypes::Sprite {
		animations: &[
			 crate::reftypes::Animation {
				frames: &[
					 crate::reftypes::Frame {
						polygons: &[
							 crate::reftypes::Polygon {
								x: &[1, 2, 3],
								y: &[4, 5, 6],
								scaled_x: &[2, 4, 6],
								scaled_y: &[4, 5, 6],
							},
						],
					},
				],
			},
			 //~ crate::reftypes::Animation {
				//~ frames: &[
					 //~ crate::reftypes::Frame {
						//~ polygons: &[
							 //~ crate::reftypes::Polygon {
								//~ x: &[7, 8, 9],
								//~ y: &[0, 0, 0],
								//~ scaled_x: &[7, 8, 9],
								//~ scaled_y: &[0, 0, 0],
							//~ },
						//~ ],
					//~ },
				//~ ],
			//~ },
		],
	};
	
	//~ pub fn new() -> crate::reftypes::Sprite {
		//~ let newspr: crate::reftypes::Sprite = crate::reftypes::Sprite {
			//~ animations: localref.animations.to_vec()
		//~ };
		//~ return newspr;
	//~ }
	
	pub fn new() -> crate::types::Sprite {
		
		let mut newspr: crate::types::Sprite = crate::types::Sprite {
			animations: Vec::new(),
		};
		
		for (_i, animation) in LOCALREF.animations.iter().enumerate() {
			
			let mut newanim: crate::types::Animation = crate::types::Animation {
				frames: Vec::new(),
			};
			
			for (_j, frame) in animation.frames.iter().enumerate(){
				
				let mut newfram: crate::types::Frame = crate::types::Frame{
					polygons: Vec::new(),
				};
				
				for (_k, polygon) in frame.polygons.iter().enumerate(){
					let newpoly: crate::types::Polygon = crate::types::Polygon {
						x:        polygon.x.to_vec(),
						y:        polygon.y.to_vec(),
						scaled_x: polygon.scaled_x.to_vec(),
						scaled_y: polygon.scaled_y.to_vec(),
					};
					
					newfram.polygons.push(newpoly);
				}
				
				newanim.frames.push(newfram);
			}
			
			newspr.animations.push(newanim);
		}
		
		return newspr;
	}
	
	//~ pub fn new() -> crate::reftypes::Sprite {
		//~ let newspr = localref;
		//~ return newspr;
	//~ }
	
}

//~ static SPRITE: SpriteData = SpriteData {
    //~ animations: &[
        //~ Animation {
            //~ frames: &[
                //~ Frame {
                    //~ polygons: &[
                        //~ Polygon {
                            //~ x: &[1, 2, 3],
                            //~ y: &[4, 5, 6],
                            //~ scaled_x: &[2, 4, 6],
                            //~ scaled_y: &[4, 5, 6],
                        //~ },
                    //~ ],
                //~ },
            //~ ],
        //~ },
        //~ Animation {
            //~ frames: &[
                //~ Frame {
                    //~ polygons: &[
                        //~ Polygon {
                            //~ x: &[1, 2, 3],
                            //~ y: &[4, 5, 6],
                            //~ scaled_x: &[1, 2, 3],
                            //~ scaled_y: &[4, 5, 6],
                        //~ },
                        //~ Polygon {
                            //~ x: &[1, 2, 3],
                            //~ y: &[4, 5, 6],
                            //~ scaled_x: &[1, 2, 3],
                            //~ scaled_y: &[4, 5, 6],
                        //~ },
                    //~ ],
                //~ },
                //~ Frame {
                    //~ polygons: &[
                        //~ Polygon {
                            //~ x: &[1, 2, 3],
                            //~ y: &[4, 5, 6],
                            //~ scaled_x: &[1, 2, 3],
                            //~ scaled_y: &[4, 5, 6],
                        //~ },
                        //~ Polygon {
                            //~ x: &[1, 2, 3],
                            //~ y: &[4, 5, 6],
                            //~ scaled_x: &[1, 2, 3],
                            //~ scaled_y: &[4, 5, 6],
                        //~ },
                    //~ ],
                //~ },
            //~ ],
        //~ },
    //~ ],
//~ };

fn draw(spr: &types::Sprite){
	println!("spr: {:?}", spr);
}

fn update(spr: &mut types::Sprite){
	
	//~ spr.animations[0].frames[0].polygons[0].scaled_x[0] = spr.animations[0].frames[0].polygons[0].scaled_x[0] + 1;
	/*
	for (i, animation) in spr.animations.iter().enumerate(){
		for (j, frame) in animation.frames.iter().enumerate(){
			for (k, polygon) in frame.polygons.iter().enumerate(){
				for (l, _) in polygon.scaled_x.iter().enumerate(){
					//~ x = x + 1;
					spr.animations[i].frames[j].polygons[k].scaled_x[l] = spr.animations[i].frames[j].polygons[k].scaled_x[l] + 1;
				}
				for (l, _) in polygon.scaled_y.iter().enumerate(){
					//~ y = y + 1;
					spr.animations[i].frames[j].polygons[k].scaled_y[l] = spr.animations[i].frames[j].polygons[k].scaled_y[l] + 1;
				}
			}
		}
	}
	*/
	
	let lenanim = spr.animations.len();
	let lenfram = spr.animations[0].frames.len();
	let lenpoly = spr.animations[0].frames[0].polygons.len();
	let lenvect = spr.animations[0].frames[0].polygons[0].x.len();
	
	for i in 0..lenanim {
		for j in 0..lenfram {
			for k in 0..lenpoly {
				for l in 0..lenvect {
					spr.animations[i].frames[j].polygons[k].scaled_x[l] = spr.animations[i].frames[j].polygons[k].scaled_x[l] + 1;
					spr.animations[i].frames[j].polygons[k].scaled_y[l] = spr.animations[i].frames[j].polygons[k].scaled_y[l] + 1;
				}
			}
		}
	}
	
}

fn main() {
	
	//~ println!("hello {}", xpbit::Ok);
	
	let mut sprite = xpbit::new();
	
	//~ let sprite = xpbit::new();
	//~ println!("{:p}", &sprite);
	
	//~ println!("sprite.animations[0].frames[0].polygons[0].x[0]: {}", sprite.animations[0].frames[0].polygons[0].x[0]);
	
	//~ sprite.animations[0].frames[0].polygons[0].x[0] = sprite.animations[0].frames[0].polygons[0].x[0] + 1;
	
	//~ println!("sprite.animations[0].frames[0].polygons[0].x[0]: {}", sprite.animations[0].frames[0].polygons[0].x[0]);
	
    println!("&sprite                                             : {:p}", &sprite                                             );
    println!("&sprite.animations                                  : {:p}", &sprite.animations                                  );
    println!("&sprite.animations[0].frames                        : {:p}", &sprite.animations[0].frames                        );
    println!("&sprite.animations[0].frames[0].polygons            : {:p}", &sprite.animations[0].frames[0].polygons            );
    println!("&sprite.animations[0].frames[0].polygons[0].x       : {:p}", &sprite.animations[0].frames[0].polygons[0].x       );
    println!("&sprite.animations[0].frames[0].polygons[0].y       : {:p}", &sprite.animations[0].frames[0].polygons[0].y       );
    println!("&sprite.animations[0].frames[0].polygons[0].scaled_x: {:p}", &sprite.animations[0].frames[0].polygons[0].scaled_x);
    println!("&sprite.animations[0].frames[0].polygons[0].scaled_y: {:p}", &sprite.animations[0].frames[0].polygons[0].scaled_y);
    //~ println!("&sprite.animations[1].frames                        : {:p}", &sprite.animations[1].frames                        );
    //~ println!("&sprite.animations[1].frames[0].polygons            : {:p}", &sprite.animations[1].frames[0].polygons            );
    //~ println!("&sprite.animations[1].frames[0].polygons[0].x       : {:p}", &sprite.animations[1].frames[0].polygons[0].x       );
    //~ println!("&sprite.animations[1].frames[0].polygons[0].y       : {:p}", &sprite.animations[1].frames[0].polygons[0].y       );
    //~ println!("&sprite.animations[1].frames[0].polygons[0].scaled_x: {:p}", &sprite.animations[1].frames[0].polygons[0].scaled_x);
    //~ println!("&sprite.animations[1].frames[0].polygons[0].scaled_y: {:p}", &sprite.animations[1].frames[0].polygons[0].scaled_y);
    
    for _ in 1..10 {
		update(&mut sprite);
		draw(&sprite);
	}
    
}
















