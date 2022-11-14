import init, { run_app } from '../pkg/app';
import "./app.css";
async function main() {
   await init('/app_bg.wasm');
   run_app();
}
main()
