use std::env;

const URL: &str = "https://opac.lib.city.yokohama.lg.jp/winj/opac/top.do";

fn showlib(user: &str, pass: &str) {
    let browser = headless_chrome::Browser::default().unwrap();
    let tab = browser.new_tab().unwrap();
    tab.set_user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36", Some("Linux x86_64"), Some("ja")).unwrap();
    tab.navigate_to(&(URL.to_string())).unwrap();

    let a = tab.wait_for_element("#box-utility > ul > li > a").unwrap();
    a.click().unwrap();

    tab.wait_for_element("#usercd").unwrap().click().unwrap();
    tab.type_str(&user).unwrap();
    tab.wait_for_element("#password").unwrap().click().unwrap();
    tab.type_str(&pass).unwrap();
    tab.press_key("Enter").unwrap();

    let g = tab.wait_for_element("#nav-guide2");
    let msg = match g {
	Ok(gok) => gok.get_inner_text().unwrap(),
	Err(_) => "No message".to_string(),
    };
    println!("{}", msg);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args[1] == "show" {
	let user = &args[2];
	let pass = &args[3];
	println!("## {}", user);
	showlib(user, pass);
	return;
    }
}
