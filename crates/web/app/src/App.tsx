import "@mantine/core/styles.css";
import { MantineProvider } from "@mantine/core";
import { theme } from "./theme";
import {init, update, render_todo} from "wasm";
import {useState, createContext} from 'react';
import Todo from "./Todo.tsx";

export const ViewWasmContext = createContext(null);
export const SetViewWasmContext = createContext(() =>  {});
const model = init();

export default function App() {
        const [view, _setView] = useState(render_todo(model));
        let setView = (message) => {
                update(model, message);
                _setView(render_todo(model));
        } ;

        /*let model = init();
        update(model, {Todo:{UpdateTaskNewTitle: "test"}});
        console.log(render_todo(model));*/
        return (
                <MantineProvider theme={theme}>
                        <ViewWasmContext.Provider value={view}>
                                <SetViewWasmContext.Provider value={setView}>
                                        <Todo />
                                </SetViewWasmContext.Provider>
                        </ViewWasmContext.Provider>
                </MantineProvider>
        );
}
