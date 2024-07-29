use leptos::*;
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Todo {
    completed: bool,
    id: u64,
    text: String,
}

// async fn fetch_todos() -> Result<Vec<Todo>, Box<dyn std::error::Error>> {
//     let res = reqwest::get("http://localhost:8000/todos").await?.text().await?;
//     let todos: Vec<Todo> = serde_json::from_str(&res)?;
//     Ok(todos)
// }

// The #[component] macro marks a function as a reusable component
// Components are the building blocks of your user interface
// They define a reusable unit of behavior
#[component]
fn ProgressBar(
    // #[prop(optional)]
    #[prop(default = 25)]
    max: u16,
    progress: impl Fn() -> i32 + 'static,
) -> impl IntoView {
    view! {
        <progress
            max=max
            // now this works
            value=progress
        />
    }
}

#[component]
fn TodoItem(todo: Todo, global_count: WriteSignal<i32>) -> impl IntoView {
    let (completed, set_completed) = create_signal(todo.completed);
    let (updating, set_updating) = create_signal(false);
    if !completed.get() {
        global_count.update(|c| *c += 1);
    }
    view! {
        <div
            style:font-size="20px"
            style:padding="10px"
        >
            <input
                style:margin-right="10px"
                type="checkbox"
                // we can use the getter and setter directly
                checked = {move || completed.get()}
                on:change = move |_| {
                    spawn_local(async move {
                        set_updating.update(|c| *c = true);
                        reqwest::get(format!("https://icp-test.fly.dev/toggle/{}", todo.id))
                            .await
                            .expect("err").text().await.expect("err");
                        if completed.get() {
                            global_count.update(|c| *c += 1);
                        } else {
                            global_count.update(|c| *c -= 1);
                        }
                        set_completed.update(|c| *c = !*c);
                        set_updating.update(|c| *c = false);
                    });
                }
            />
            <span
                style:color=move || {
                    if completed.get() && !updating.get() {
                        "green"
                    } else if updating.get() {
                        "black"
                    } else {
                        "black"
                    }
                }
                // strike through the text if the todo is completed
                style:text-decoration=move || {
                    if completed.get() && !updating.get() {
                        // completed and not updating
                        "none"
                    } else if updating.get() {
                        "none"
                    } else {
                        "underline"
                    }
                }
            >
                {move || {
                    if updating.get() {
                        "⏳  "
                    } else {
                        ""
                    }
                }}
                {todo.text}
            </span>
        </div>
    }
}

#[component]
fn App() -> impl IntoView {
    // here we create a reactive signal
    // and get a (getter, setter) pair
    // signals are the basic unit of change in the framework
    // we'll talk more about them later
    let (count, set_count) = create_signal(0);
    let double_count = move || count.get() * 2;

    set_count.set(4);

    let (completed_count, set_completed_count) = create_signal(0);
    // set_completed_count.set(0);

    // our resource
    let async_data = create_resource(
        move || count.get(),
        // every time `count` changes, this will run
        |value| async move {
            // logging::log!("loading data from API");
            // let todos = reqwest::get("http://localhost:8000/todos")
            //     .await
            //     .expect("err")
            //     .text()
            //     .await
            //     .expect("err");
            // let todos: Vec<Todo> = serde_json::from_str(&todos).expect("err");
            // logging::log!("data loaded from API");
            // todos
            let hardcoded_data = vec![
                Todo {
                    completed: false,
                    id: 0,
                    text: "Test todo 1".to_string(),
                },
                Todo {
                    completed: true,
                    id: 1,
                    text: "Test todo 2".to_string(),
                },
                Todo {
                    completed: false,
                    id: 2,
                    text: "Test todo 3".to_string(),
                },
            ];
            return hardcoded_data;
        },
    );
    let async_result = move || {
        async_data
            .get()
            .map(|value| format!("Server returned {value:?}"))
            // This loading state will only show before the first load
            .unwrap_or_else(|| "Loading...".into())
    };
    let todos: Resource<(), Vec<Todo>> = create_resource(|| (), |_| async {
        logging::log!("loading data from API");
        // let todos = reqwest::get("http://127.0.0.1:8081/todos").await.expect("err").text().await.expect("err");
        let todos = reqwest::get("https://icp-test.fly.dev/todos").await.expect("err").text().await.expect("err");
        let todos: Vec<Todo> = serde_json::from_str(&todos).expect("err");
        logging::log!("data loaded from API");
        todos
        // println!("loading data from hardcoded source");
        // vec![
        //     Todo {
        //         completed: true,
        //         id: 0,
        //         text: "Rust ic_agent to fetch Todos from canister".to_string(),
        //     },
        //     Todo {
        //         completed: true,
        //         id: 1,
        //         text: "Integrate Axum server and deploy to fly.io".to_string(),
        //     },
        //     Todo {
        //         completed: false,
        //         id: 2,
        //         text: "Add CORS to backend server on fly.io".to_string(),
        //     },
        //     Todo {
        //         completed: true,
        //         id: 3,
        //         text: "Make the text and UI reactive".to_string(),
        //     },
        //     Todo {
        //         completed: false,
        //         id: 4,
        //         text: "Show what I made to Saikat and Komal".to_string(),
        //     },
        //     Todo {
        //         completed: false,
        //         id: 5,
        //         text: "Add trigger to toggle/update todo in canister".to_string(),
        //     },
        //     Todo {
        //         completed: false,
        //         id: 6,
        //         text: "Add refresh button, and auto-refresh every 10 seconds".to_string(),
        //     },
        //     Todo {
        //         completed: false,
        //         id: 7,
        //         text: "Remove the test stuff below".to_string(),
        //     },
        // ] as Vec<Todo>
    });

    // let todos_length = move |set_completed_count: WriteSignal<i32>| {
    //     match todos.get() {
    //         Some(todos) => {
    //             let mut left = 0;
    //             for todo in todos.iter() {
    //                 if !todo.completed {
    //                     left += 1;
    //                 }
    //             }
    //             set_completed_count.update(|c| *c = left);
    //             left.to_string()
    //         },
    //         None => "Loading...".to_string(),
    //     }
    // };

    let todos_display = move || {
        match todos.get() {
            Some(todos) => {
                todos.iter().map(|todo| view! { <TodoItem todo=todo.clone() global_count=set_completed_count/> }).collect::<Vec<_>>()
            }
            None => vec![view! { <p>"Loading..."</p> }.into_view()],
        }
    };

    // the `view` macro is how we define the user interface
    // it uses an HTML-like format that can accept certain Rust values
    view! {
        <h1>"Things to do:"</h1>
        <p>
            // <code>"stable"</code>": " {move || stable.get()}
            <h2>
            {move || match completed_count.get() {
                0 => "All done, enjoy the day!".to_string(),
                1 => "Just one more item".to_string(),
                n => format!("{} things left to do", n),
            }}
            </h2>
        </p>
        <div>
            // we can use the getter directly
            // this will reactively update
            {todos_display}
        </div>

        <br/>
        <br/>
        <br/>
        <br/>
        <br/>
        <br/>

        <h4>"Some random test stuff"</h4>
        <button
            // on:click will run whenever the `click` event fires
            // every event handler is defined as `on:{eventname}`

            // we're able to move `set_count` into the closure
            // because signals are Copy and 'static
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }
            style:padding=move || format!("{}px", count.get() * 2)
            // class:red=move || count.get() % 2 == 1
            class=("red", move || count.get() % 2 == 1)
        >
            // text nodes in RSX should be wrapped in quotes,
            // like a normal Rust string
            {move || format!("Clicks: {}", count.get())}
        </button>
        <br/>
        // <progress
        //     max="25"
        //     // signals are functions, so `value=count` and `value=move || count.get()`
        //     // are interchangeable.
        //     value=count
        // />
        <ProgressBar progress = move || count.get()/>
        <br/>
        <ProgressBar progress = double_count/>
        // <p>
        //     <strong>"Reactive: "</strong>
        //     // you can insert Rust expressions as values in the DOM
        //     // by wrapping them in curly braces
        //     // if you pass in a function, it will reactively update
        //     {move || count.get()}
        // </p>
        // <p>
        //     <strong>"Reactive shorthand: "</strong>
        //     // signals are functions, so we can remove the wrapping closure
        //     {count}
        // </p>

        // <p>
        //     <strong>"Not reactive: "</strong>
        //     // NOTE: if you write {count()}, this will *not* be reactive
        //     // it simply gets the value of count once
        //     {count.get()}
        // </p>
        // <p>
        //     <code>"async_value"</code>": "
        //     {async_result}
        //     <br/>
        //     // {is_loading}
        // </p>
    }
}


fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}