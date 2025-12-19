import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { ThemeProvider, useTheme } from "./components/theme-provider";
import { Blockquote, H2, Muted, P } from "./components/ui/typography";
import { Badge } from "./components/ui/badge";
import { Button } from "./components/ui/button";
import { Moon, Sun } from "lucide-react";
import { columns, TodoItem } from "./components/columns";
import { DataTable } from "./components/data-table";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setGreetMsg(await invoke("greet", { name }));
  }

  const { theme, setTheme } = useTheme()
  const toggleTheme = () => {
    setTheme(theme === "dark" ? "light" : "dark");
  };

  const data: TodoItem[] = [
    {
      id: 1,
      title: "Test item Test item Test item",
      content: "Some long sample content. Some long sample content. Some long sample content. Some long sample content.",
      done: false,
      priority: 0,
      creation_date: 1,
      due_date: 2,
      finish_date: 3,
    },
    {
      id: 2,
      title: "Test item 2",
      content: "short",
      done: true,
      priority: 10,
      creation_date: 2,
      due_date: 2,
      finish_date: 2,
    }
  ]

  return (
    <main className="m-5">
      <div className="flex flex-col mb-10">
        <div className="flex flex-row justify-between items-center">
          <div className="flex flex-row items-baseline gap-2">
            <H2 className="mb-2">tick</H2>
            <P>off your personal todos.</P>
          </div>
          <Button variant="outline" size="icon" onClick={() => toggleTheme()} className="ml-10">
            <Sun className="h-[1.2rem] w-[1.2rem] scale-100 rotate-0 transition-all dark:scale-0 dark:-rotate-90" />
            <Moon className="absolute h-[1.2rem] w-[1.2rem] scale-0 rotate-90 transition-all dark:scale-100 dark:rotate-0" />
          </Button>
        </div>
        <div className="flex flex-row items-center gap-1">
          <Muted>Maybe start with</Muted>
          <Badge variant="outline" className="bg-(--primary)">
            <button>
              <Muted>Some item</Muted>
            </button>
          </Badge>
        </div>
      </div>

      <DataTable columns={columns} data={data} />

      {/*<h1>Welcome to Tauri + React</h1>

      <div className="row">
        <a href="https://vite.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://react.dev" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <p>Click on the Tauri, Vite, and React logos to learn more.</p>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Greet</button>
      </form>
      <p>{greetMsg}</p>*/}
    </main>
  );
}

export default App;
