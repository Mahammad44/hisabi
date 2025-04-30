use leptos::view; // Explicit view macro import
use leptos::*;
use crate::models::ZakatResponse;

#[component]
pub fn App() -> impl IntoView {
    let (cash, set_cash) = create_signal(0.0);
    let (gold, set_gold) = create_signal(0.0);
    let (silver, set_silver) = create_signal(0.0);
    let (debts, set_debts) = create_signal(0.0);
    let (result, set_result) = create_signal(None::<ZakatResponse>);

    let submit = move |_| {
        spawn_local(async move {
            let response = reqwest::Client::new()
                .post("http://localhost:3000/api/calculate")
                .json(&serde_json::json!({
                    "cash": cash.get(),
                    "gold_grams": gold.get(),
                    "silver_grams": silver.get(),
                    "debts": debts.get()
                }))
                .send()
                .await
                .unwrap()
                .json::<ZakatResponse>()
                .await
                .unwrap();

            set_result.set(Some(response));
        });
    };

    view! {
        <main class="max-w-2xl mx-auto p-4">
            <h1 class="text-3xl font-bold mb-4">"Hisabi Zakat Calculator"</h1>
            <form on:submit=submit class="space-y-4">
                <div class="grid grid-cols-2 gap-4">
                    <div>
                        <label class="block mb-2">"Cash & Savings"</label>
                        <input
                            type="number"
                            step="0.01"
                            class="w-full p-2 border rounded"
                            on:input=move |ev| set_cash.set(
                                event_target_value(&ev).parse().unwrap_or(0.0)
                            )
                        />
                    </div>
                    <div>
                        <label class="block mb-2">"Gold (grams)"</label>
                        <input
                            type="number"
                            step="0.01"
                            class="w-full p-2 border rounded"
                            on:input=move |ev| set_gold.set(
                                event_target_value(&ev).parse().unwrap_or(0.0)
                            )
                        />
                    </div>
                    <div>
                        <label class="block mb-2">"Silver (grams)"</label>
                        <input
                            type="number"
                            step="0.01"
                            class="w-full p-2 border rounded"
                            on:input=move |ev| set_silver.set(
                                event_target_value(&ev).parse().unwrap_or(0.0)
                            )
                        />
                    </div>
                    <div>
                        <label class="block mb-2">"Debts"</label>
                        <input
                            type="number"
                            step="0.01"
                            class="w-full p-2 border rounded"
                            on:input=move |ev| set_debts.set(
                                event_target_value(&ev).parse().unwrap_or(0.0)
                            )
                        />
                    </div>
                </div>
                <button
                    type="submit"
                    class="w-full bg-green-500 text-white p-2 rounded hover:bg-green-600"
                >
                    "Calculate Zakat"
                </button>
            </form>

            {move || result.get().map(|res| view! {
                <div class="mt-4 p-4 bg-gray-100 rounded">
                    <p class="text-lg font-semibold">{res.message.clone()}</p>
                    <p class="text-2xl mt-2">
                        {format!("Amount Due: ${:.2}", res.zakat_due)}
                    </p>
                </div>
            })}
        </main>
    }
}
