import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { useTheme } from "./components/theme-provider";
import { H2, P } from "./components/ui/typography";
import { Button } from "./components/ui/button";
import { Menu, Moon, Sun } from "lucide-react";
import { columns, TodoItem } from "./components/columns";
import { DataTable, QueryParams } from "./components/data-table";
import { toast } from "sonner";
import { SortingState } from "@tanstack/react-table";
import { Sheet, SheetContent } from "./components/ui/sheet";
import { Pagination, PaginationContent, PaginationItem, PaginationLink, PaginationNext, PaginationPrevious } from "./components/ui/pagination";

function App() {
  const { theme, setTheme } = useTheme()
  const toggleTheme = () => {
    setTheme(theme === "dark" ? "light" : "dark");
  };

  const [todos, setTodos] = useState<TodoItem[]>([]);
  const [sorting, setSorting] = useState<SortingState>([]);
  const [menuOpen, setMenuOpen] = useState(false);
  const [statusFilter, setStatusFilter] = useState<boolean | undefined>(undefined);
  const [searchString, setSearchString] = useState<string | undefined>(undefined);

  const PAGE_SIZE = 25;
  const [page, setPage] = useState(0);

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

      const result = await invoke<TodoItem[]>("fetch_todos", {
        params: {
          ...query,
          count: PAGE_SIZE,
          offset: PAGE_SIZE * page,
          done: statusFilter,
          search: searchString,
        },
      })

      setTodos(result)
    } catch (err: any) {
      setTodos([])
      toast.error(err.toString())
    }
  };

  useEffect(() => {
    fetchTodos()
  }, [sorting, page, statusFilter, searchString])

  //reset pagination on sorting
  useEffect(() => {
    setPage(0);
  }, [sorting, statusFilter, searchString]);

  return (
    <main className="m-5 h-[calc(100vh-2.5rem)] flex flex-col">
      <div className="flex flex-col mb-2">
        <div className="flex flex-row justify-between items-center">
          <div className="flex flex-row items-baseline gap-2">
            <H2 className="mb-2">tick</H2>
            <P>off your personal todos.</P>
          </div>
          <div className="flex flex-row gap-3">
            <Button variant="outline" size="icon-sm" onClick={() => toggleTheme()}>
              <Sun className="h-[1.2rem] w-[1.2rem] scale-100 rotate-0 transition-all dark:scale-0 dark:-rotate-90" />
              <Moon className="absolute h-[1.2rem] w-[1.2rem] scale-0 rotate-90 transition-all dark:scale-100 dark:rotate-0" />
            </Button>
            <Button variant="outline" size="icon-sm" onClick={() => setMenuOpen(true)}>
              <Menu/>
            </Button>
          </div>
        </div>
      </div>

      <div className="flex-1 min-h-0">
        <DataTable
          columns={columns(fetchTodos)}
          data={todos}
          sorting={sorting}
          setSorting={setSorting}
          fetchTodos={fetchTodos}
          statusFilter={statusFilter}
          setStatusFilter={setStatusFilter}
          searchString={searchString}
          setSearchString={setSearchString}
        />
      </div>

      <Pagination className="justify-end mt-2">
        <PaginationContent>
          <PaginationItem>
            <PaginationPrevious
              onClick={(e) => {
                e.preventDefault();
                setPage((p) => Math.max(0, p - 1));
              }}
              aria-disabled={page === 0}
              className={page === 0 ? "pointer-events-none opacity-50" : ""}
            />
          </PaginationItem>

          {page == 0 && (
            <>
              <PaginationItem>
                <PaginationLink isActive 
                  onClick={(e) => {
                    e.preventDefault();
                    setPage(() => 0);
                  }}>
                  1
                </PaginationLink>
              </PaginationItem>

              <PaginationItem>
                <PaginationLink
                  onClick={(e) => {
                    e.preventDefault();
                    setPage(() => 1);
                  }}>
                  2
                </PaginationLink>
              </PaginationItem>

              <PaginationItem>
                <PaginationLink>
                  3
                </PaginationLink>
              </PaginationItem>
            </>
          )}
  
          {page >= 1 && (
            <>
              <PaginationItem>
                <PaginationLink
                  onClick={(e) => {
                    e.preventDefault();
                    setPage((p) => p - 1);
                  }}>
                  {page}
                </PaginationLink>
              </PaginationItem>

              <PaginationItem>
                <PaginationLink isActive
                  onClick={(e) => {
                    e.preventDefault();
                    setPage((p) => p);
                  }}>
                  {page + 1}
                </PaginationLink>
              </PaginationItem>

              <PaginationItem>
                <PaginationLink
                  onClick={(e) => {
                    e.preventDefault();
                    setPage((p) => p + 1);
                  }}>
                  {page + 2}
                </PaginationLink>
              </PaginationItem>
            </>
          )}

          {/* <PaginationItem>
            <PaginationEllipsis />
          </PaginationItem> */}

          <PaginationItem>
            <PaginationNext
              onClick={(e) => {
                e.preventDefault();
                setPage((p) => p + 1);
              }}
            />
          </PaginationItem>
        </PaginationContent>
      </Pagination>
      
      <Sheet open={menuOpen} onOpenChange={setMenuOpen}>
        <SheetContent>
        </SheetContent>   
      </Sheet>
    </main>
  );
}

export default App;
