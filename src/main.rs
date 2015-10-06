extern crate gtk;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

use gtk::traits::*;
use gtk::signal::Inhibit;
use gtk::{IconSize, Orientation, ReliefStyle};

struct NoteBook {
	notebook: gtk::NoteBook,
	tabs: Vec<gtk::Box>
}

impl NoteBook{
	
	fn new() -> NoteBook{
		NoteBook{
			notebook: gtk::NoteBook::new().unwrap(),
			tabs: Vec::new()
		}
	}
	
	fn create_tab<'a, Widget>(&mut self, title: &'a str, widget: &Widget) -> Option<u32>
	where Widget: gtk::WidgetTrait + Clone + 'static {
		let close_image = gtk::Image::new_from_icon_name("window-close", IconSize::Button as i32).unwrap();
		let button = gtk::Button::new().unwrap();
		let label = gtk::Label::new(title).unwrap();
		let tab = gtk::Box::new(Orientation::Horizontal, 0).unwrap();
		
		button.set_relief(ReliefStyle::None);
		button.set_focus_on_click(false);
		button.add(&close_image);
		
		tab.pack_start(&label, false, false, 0);
		tab.pack_start(&button, false, false, 0);
		tab.show_all();
		
		let index = match self.notebook.append_page(widget, Some(&tab)){
			Some(index) => index,
			_ => return None
		};
		
		let notebook_clone = self.notebook.clone();
		let widget_clone = widget.clone();
		button.connect_clicked(move |_| {
			let index = notebook_clone.page_num(&widget_clone).unwrap();
			notebook_clone.remove_page(index as i32);
		});
		
		self.tabs.push(tab);
		
		Some(index)
	}

}

fn main() {

	gtk::init().unwrap_or_else(|_| panic!("Failed to init gtk."));
	
	let window = gtk::Window::new(gtk::WindowType::Toplevel).unwrap();
	window.set_title("Text File Viewer");
	window.set_window_position(gtk::WindowPosition::Center);
	window.set_default_size(800, 500);
	
	//~ let toolbar = gtk::Toolbar::new().unwrap();
	
	//call icons
	//~ let open_icon = gtk::Image::new_from_icon_name("document-open", gtk::IconSize::SmallToolbar as i32).unwrap();
	//~ let save_icon = gtk::Image::new_from_icon_name("document-save", gtk::IconSize::SmallToolbar as i32).unwrap();
	//~ let build_icon = gtk::Image::new_from_icon_name("system-run", gtk::IconSize::SmallToolbar as i32).unwrap();
	//~ let run_icon = gtk::Image::new_from_icon_name("media-playback-start", gtk::IconSize::SmallToolbar as i32).unwrap();
	
	//text view
	//~ let text_view = gtk::TextView::new().unwrap();
	
	//~ //open button
	//~ let open_button = gtk::ToolButton::new::<gtk::Image>(Some(&open_icon), Some("Open")).unwrap();
	//~ open_button.set_is_important(true);
	//~ toolbar.add(&open_button);
	//~ 
	//~ //save button
	//~ let save_button = gtk::ToolButton::new::<gtk::Image>(Some(&save_icon), Some("Save")).unwrap();
	//~ save_button.set_is_important(true);
	//~ toolbar.add(&save_button);
	//~ 
	//~ //build button
	//~ let build_button = gtk::ToolButton::new::<gtk::Image>(Some(&build_icon), Some("Build")).unwrap();
	//~ build_button.set_is_important(true);
	//~ toolbar.add(&build_button);
	//~ 
	//~ //run button
	//~ let run_button = gtk::ToolButton::new::<gtk::Image>(Some(&run_icon), Some("Run")).unwrap();
	//~ run_button.set_is_important(true);
	//~ toolbar.add(&run_button);
	//~ 
	//~ //scroll
	//~ let scroll = gtk::ScrolledWindow::new(None, None).unwrap();
	//~ scroll.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
	//~ scroll.add(&text_view);
	//~ 
	//toolBOT
	//~ let bot_frame = gtk::Frame::new(Some("Status:")).unwrap();
	//~ bot_frame.set_border_width(10);
	//~ 
	//~ //vertbox
	//~ let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0).unwrap();
	//~ vbox.pack_start(&toolbar, false, true, 0);
	//~ vbox.pack_start(&scroll, true, true, 0);
	//~ vbox.pack_start(&bot_frame, false, true, 0);
	//~ 
	//~ window.add(&vbox);
	//~ 
	//~ //open operations
	//~ open_button.connect_clicked(move |_| {
		//~ let file_chooser = gtk::FileChooserDialog::new(
			//~ "Open File", None, gtk::FileChooserAction::Open,
			//~ [("Open", gtk::ResponseType::Ok), ("Cancel", gtk::ResponseType::Cancel)]);
		//~ if file_chooser.run() == gtk::ResponseType::Ok as i32 {
			//~ let filename = file_chooser.get_filename().unwrap();
			//~ let file = File::open(&filename).unwrap();
			//~ 
			//~ let mut reader = BufReader::new(file);
			//~ let mut contents = String::new();
			//~ let _ = reader.read_to_string(&mut contents);
			//~ 
			//~ text_view.get_buffer().unwrap().set_text(&contents);
		//~ }
		//~ 
		//~ file_chooser.destroy();
	//~ });
	//~ 
	//~ //save operations
	//~ save_button.connect_clicked(move |_| {
		//~ let file_chooser = gtk::FileChooserDialog::new(
			//~ "Save File", None, gtk::FileChooserAction::Save,
			//~ [("Save", gtk::ResponseType::Ok), ("Cancel", gtk::ResponseType::Cancel)]);
		//~ if file_chooser.run() == gtk::ResponseType::Ok as i32 {
			//~ let filename = file_chooser.get_filename().unwrap();
			//~ let file = File::save(&filename).unwrap();
			
			//~ let mut reader = BufReader::new(file);
			//~ let mut contents = String::new();
			//~ let _ = reader.read_to_string(&mut contents);
			//~ 
			//~ text_view.get_buffer().unwrap().set_text(&contents);
		//~ }
		//~ 
		//~ file_chooser.destroy();
	//~ });
	
	window.connect_delete_event(|_,_| {
		gtk::main_quit();
		Inhibit(true)
	});
	
	let mut notebook = NoteBook::new();
	
	for i in 1..4 {
		let title = format!("sheet {}", i);
		let label = gtk::Label::new(&title[..]).unwrap();
		notebook.create_tab(&title[..], &label);
	}
	
	window.add(&notebook.notebook);
	window.show_all();
	gtk::main();

}
