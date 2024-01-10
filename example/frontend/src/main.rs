use gloo_net::http::{Request, Method};
use task_core::Task;
use web_sys::{wasm_bindgen::JsCast, HtmlInputElement, HtmlElement};
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let input_ref = use_node_ref();

    let todo = use_state(|| Vec::<Task>::new());

    // Fetch list
    {
        let todo = todo.clone();
        use_effect_with((), move |_| {
            let todo = todo.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let tasks = Request::get("http://localhost:8001/tasks")
                    .send()
                    .await
                    .expect("Could not complete request /tasks")
                    .json()
                    .await
                    .expect("Could not parse json /tasks");
                todo.set(tasks);
            });
            || ()
        });
    }

    let input = use_state(|| "".to_string());
    let handle_text_input = {
        let input = input.clone();
        move |input_event: Event| {
            let ele: HtmlInputElement = input_event
                .target()
                .unwrap()
                .dyn_into::<HtmlInputElement>()
                .unwrap();
            let s = ele.value();
            input.set(s);
        }
    };

    let handle_add_task = {
        let input = input.clone();
        let todo = todo.clone();
        let input_ref = input_ref.clone();
        move |_| {
            let input_ele = input_ref.cast::<HtmlInputElement>().unwrap();
            input_ele.set_value("");
            let value = input.to_string();
            if value.len() == 0 {
                return;
            }
            let task = Task::new(value);
            let mut list = todo.to_vec();
            {
                let task = task.clone();
                let todo = todo.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let tasks = Request::post("http://localhost:8001/task")
                        .json(&task)
                        .expect("Could not serialize task")
                        .send()
                        .await
                        .expect("Could not complete request /tasks")
                        .json()
                        .await
                        .expect("Could not parse json /tasks");
                    todo.set(tasks);
                });
            }
            list.push(task);
            todo.set(list);
        }
    };

    let handle_complete_task = {
        let todo = todo.clone();
        move |event: Event| {
            let ele: HtmlInputElement = event
                .target()
                .unwrap()
                .dyn_into::<HtmlInputElement>()
                .unwrap();
            let task_id = ele.parent_element().unwrap().id();
            // The task index is in the id with the format task_id
            let id = task_id.split("_").last().unwrap().parse::<usize>().unwrap();
            let mut list = todo.to_vec();
            if let Some(task) = list.get_mut(id) {
                task.done = ele.checked();
                {
                    let task = task.clone();
                    let todo = todo.clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        let tasks = Request::post(&format!("http://localhost:8001/task/{}", id))
                            .method(Method::PUT)
                            .json(&task)
                            .expect("Could not serialize task")
                            .send()
                            .await
                            .expect("Could not complete request /task")
                            .json()
                            .await
                            .expect("Could not parse json /task");
                        todo.set(tasks);
                    });
                }
            }
            todo.set(list);
        }
    };

    let handle_delete_task = {
        let todo = todo.clone();
        move |event: MouseEvent| {
            let ele: HtmlElement = event
                .target()
                .unwrap()
                .dyn_into::<HtmlElement>()
                .unwrap();
            let task_id = ele.parent_element().unwrap().id();
            // The task index is in the id with the format task_id
            let id = task_id.split("_").last().unwrap().parse::<usize>().unwrap();
            let mut list = todo.to_vec();
            if let Some(_) = list.get_mut(id) {
                list.remove(id);
                {
                    let todo = todo.clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        let tasks = Request::delete(&format!("http://localhost:8001/task/{}", id))
                            .send()
                            .await
                            .expect("Could not complete request /task")
                            .json()
                            .await
                            .expect("Could not parse json /task");
                        todo.set(tasks);
                    });
                }
            }
            todo.set(list);
        }
    };

    let list_html = if todo.len() == 0 {
        html!(
            <p>{ "No tasks, congrats!" }</p>
        )
    } else {
        todo.iter().enumerate()
            .map(|(index, task)| {
                let delete_button = if task.done {
                    Some(html!(
                        <button class="py-1 px-2 border rounded border-neutral-600" onclick={handle_delete_task.clone()}>
                            { "Delete" }
                        </button>
                    ))
                } else {
                    None
                };
                html!(
                    <div id={format!("task_{}", index)} class="flex gap-3 w-96">
                        <p class="w-full">{ &*task.message }</p>
                        { delete_button }
                        <input type="checkbox" checked={task.done} onchange={handle_complete_task.clone()}/>
                    </div>
                )
            })
            .collect::<Html>()
    };

    html! {
        <div class={ "container mx-auto flex flex-col items-center justify-center h-screen" }>
            <div class="flex flex-col gap-2 p-2 flex-1 justify-end overflow-y-scroll">
                { list_html }
            </div>
            <div class="flex gap-2 m-4">
                <input ref={ input_ref } onchange={handle_text_input} class="py-1 px-2 border rounded border-neutral-600" type="text"/>
                <button onclick={handle_add_task} class="py-1 px-2 border rounded border-neutral-600">
                    { "Add Task" }
                </button>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
