use axum::{extract::Path, routing::get, Router};

async fn day1(Path(nums): Path<String>) -> Result<String, String> {
    let params = nums
        .split('/')
        .map(|n| n.parse())
        .collect::<Vec<Result<i32, _>>>();

    if let Some(Err(err)) = params.iter().find(|p| p.is_err()) {
        return Err(format!("Found error: {err}"));
    }

    let nums = params.into_iter().map(|p| p.unwrap());

    let num = nums
        .into_iter()
        .reduce(|acc, n| acc ^ n)
        .ok_or_else(|| "Failed to perform the ^ operation".to_string())?;

    Ok(num.pow(3).to_string())
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route("/1/*nums", get(day1));

    Ok(router.into())
}
