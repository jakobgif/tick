import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { useTheme } from "./components/theme-provider";
import { H2, Muted, P } from "./components/ui/typography";
import { Badge } from "./components/ui/badge";
import { Button } from "./components/ui/button";
import { Moon, Sun } from "lucide-react";
import { columns, TodoItem } from "./components/columns";
import { DataTable, QueryParams } from "./components/data-table";
import { toast } from "sonner";
import { SortingState } from "@tanstack/react-table";

function App() {
  const { theme, setTheme } = useTheme()
  const toggleTheme = () => {
    setTheme(theme === "dark" ? "light" : "dark");
  };

  const [todos, setTodos] = useState<TodoItem[]>([]);
  const [sorting, setSorting] = useState<SortingState>([])

  function mapSortingToQuery(sorting: SortingState): Pick<QueryParams, "sort_by" | "order"> {
    if (!sorting.length) return {}

    const { id, desc } = sorting[0]

    return {
      sort_by: id as any,
      order: desc ? "desc" : "asc",
    }
  }

  const fetchTodos = async () => {
    try {
      const query = mapSortingToQuery(sorting)

      toast.info(JSON.stringify(query));

      const result = await invoke<TodoItem[]>("fetch_todos", {
        params: {
          ...query,
          count: 25,
          offset: 0,
        },
      })

      setTodos(result)
    } catch (err: any) {
      toast.error(err.toString())
    }
  };

  useEffect(() => {
    fetchTodos()
  }, [sorting])

  return (
    <main className="m-5">
      <div className="flex flex-col mb-10">
        <div className="flex flex-row justify-between items-center">
          <div className="flex flex-row items-baseline gap-2">
            <H2 className="mb-2">tick</H2>
            <P>off your personal todos.</P>
          </div>
          <Button variant="outline" size="icon-sm" onClick={() => toggleTheme()} className="ml-10">
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
      
      <DataTable
        columns={columns(fetchTodos)}
        data={todos}
        sorting={sorting}
        setSorting={setSorting}
      />

      <Button onClick={fetchTodos}>Refresh Todos</Button>
    </main>
  );
}

export default App;
