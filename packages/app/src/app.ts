import init, { run_app } from '../pkg/app';
import "./app.css";
async function main() {
    window.addEventListener('keydown', (ev) => {
      if (ev.key === 'ArrowUp' || ev.key === 'ArrowDown') {
         ev.preventDefault();
       }
    }, false)
   const here = new URL(window.location.href)
   await init(here.pathname + 'app_bg.wasm');
   run_app();
}
main()
