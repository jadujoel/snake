import init, { run_app } from '../pkg/app';
import "./app.css";
async function main() {
   const here = new URL(window.location.href)
   await init(here.pathname + 'app_bg.wasm');
   run_app();
}
main()
