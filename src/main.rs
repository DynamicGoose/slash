use slint::{ComponentHandle, ModelRc, VecModel};

slint::include_modules!();

fn main() {
    let handle = MainWindow::new().unwrap();

    let search_handle = handle.as_weak();
    handle.global::<Logic>().on_search(move || {
        let search_text = search_handle.unwrap().global::<Logic>().get_searchText();
        let search_handle = search_handle.clone();
        std::thread::spawn(move || {
            slint::invoke_from_event_loop(move || {
                search_handle
                    .unwrap()
                    .global::<Logic>()
                    .set_mainViewData(ModelRc::new(VecModel::from(vec![
                        "searched for:".into(),
                        search_text.into(),
                    ])))
            })
            .unwrap();
        });
    });

    handle.run().unwrap();
}
