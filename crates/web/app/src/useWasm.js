import {init, update as updateWasm, render_todo} from "wasm";
import {useSyncExternalStore} from "react";

const model = init();

export const View = {
        Todo: {
                data: Object.freeze(render_todo(model)),
                render: render_todo,
                listeners: new Set(),
                getSnapshot: () => View.Todo.data,
                subscribe: (listener) => subscribe(View.Todo, listener),
                update: (message) => update(View.Todo, {Todo: message}),
        }
};

const subscribe = (view, listener) => {
        view.listeners.add(listener);
        console.log({Subscribe: View});
        return () => view.listeners.delete(listener);
}
const update = (view, message) => {
        updateWasm(model, message);
        view.data = Object.freeze(view.render(model));
        view.listeners.forEach(listener => listener());
        console.log({Update: View});
}

export const useWasm = (view) => useSyncExternalStore(view.subscribe, view.getSnapshot);
