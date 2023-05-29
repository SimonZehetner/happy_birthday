use gloo::timers::callback::Interval;
use js_sys::Date;
use num_format::{Locale, ToFormattedString};
use yew::prelude::*;

#[function_component(Header)]
fn header() -> Html {
    html! {
        <div class="happy-birthday-message">
            <div class="birthday-text-wrapper">
                <img class="birthday-text pop-up" src="public/Happy_Birthday_Text_small.png" alt="Happy Birthday!" />
            </div>
            // <span class="subtitle">{ "from Simon with " }<i class="heart" /></span>
            <span class="subtitle pop-up">{ "from Simon" }</span>
        </div>
    }
}

enum Msg {
    UpdateTimer,
}

#[derive(Properties, PartialEq)]
struct OldAfProps {
    title: AttrValue,
    birthday: Date,
}

struct OldAF {
    time: Date,
    _interval: Interval,
}

fn get_current_time() -> Date {
    Date::new_0()
}

impl Component for OldAF {
    type Message = Msg;
    type Properties = OldAfProps;

    fn create(ctx: &Context<Self>) -> Self {
        let update_timer = {
            let link = ctx.link().clone();
            Interval::new(100, move || link.send_message(Msg::UpdateTimer))
        };
        Self {
            time: get_current_time(),
            _interval: update_timer,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateTimer => {
                self.time = get_current_time();
                true
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let birthday = ctx.props().birthday.clone();
        let birthday_timestamp = birthday.get_time();
        let current_timestamp = self.time.get_time();

        let difference_seconds = ((current_timestamp - birthday_timestamp) / 1000.0) as u32;
        let difference_minutes = difference_seconds / 60;
        let difference_hours = difference_minutes / 60;
        let difference_days = difference_hours / 24;
        let difference_weeks = difference_days / 7;
        let mut difference_years =
            self.time.get_full_year() - birthday.get_full_year();
        let difference_months = (difference_years * 12) - birthday.get_month() + self.time.get_month();
        if birthday.get_month() > self.time.get_month()
            || (birthday.get_month() == self.time.get_month()
                && birthday.get_date() > self.time.get_date())
        {
            difference_years -= 1;
        }

        html! {
            <div class="old-af-wrapper">
                <span class="old-af-header">{ ctx.props().title.clone() }</span>
                <div class="old-af-content">
                    <span>{ difference_seconds.to_formatted_string(&Locale::de) } {" Sekunden"}</span>
                    <span>{ difference_minutes.to_formatted_string(&Locale::de) } {" Minuten"}</span>
                    <span>{ difference_hours.to_formatted_string(&Locale::de) } {" Stunden"}</span>
                    <span>{ difference_days.to_formatted_string(&Locale::de) } {" Tage"}</span>
                    <span>{ difference_weeks.to_formatted_string(&Locale::de) } {" Wochen"}</span>
                    <span>{ difference_months } {" Monate"}</span>
                    <span>{ difference_years } {" Jahre"}</span>
                </div>
            </div>
        }
    }
}

#[function_component(Cake)]
fn cake() -> Html {
    html! {
        <div class="cake-wrapper">
            <div class="old-af-wrapper">
                <span>{ "Noch so fü Verzweiflung mit CSS bin i a amoi so gemein wie du und nimm ma a \"klanes\" Stück fa dein Geburtstagskuchen" }</span>
            <img style={ "max-width: 100%;" } src={ "public/3KE.gif" }/>
            </div>
        </div>
    }
}

#[function_component(Footer)]
fn footer() -> Html {
    html! {
        <footer style="margin-top: 1em; font-size: 1rem;">
            { "Made with viel Liebe, Yew (Rust) und leider CSS" }
        </footer>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <Header/>
            <OldAF title={"Holy Shit bist du scho oid:"} birthday={Date::new_with_year_month_day(1993, 5, 1)}/>
            <OldAF title={"Zeit seit Herr der Ringe schaun:"} birthday={Date::new_with_year_month_day_hr(2022, 10, 1, 18)}/>
            <Cake/>
            <Footer/>
        </main>
    }
}
