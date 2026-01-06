import {
  ColumnDef,
  flexRender,
  getCoreRowModel,
  SortingState,
  useReactTable,
} from "@tanstack/react-table"

import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table"
import { DataTableToolbar } from "./data-table-toolbar"
import { useState } from "react";
import { TodoItem } from "./columns";
import { TodoItemDialog } from "./todo-item-dialog";
import { AppConfig } from "@/lib/app-config";

interface DataTableProps {
  columns: ColumnDef<TodoItem, any>[]
  data: TodoItem[]
  sorting: SortingState
  setSorting: React.Dispatch<React.SetStateAction<SortingState>>
  fetchTodos: () => Promise<void>
  statusFilter: boolean | undefined
  setStatusFilter: React.Dispatch<React.SetStateAction<boolean | undefined>>
  searchString: string | undefined
  setSearchString: React.Dispatch<React.SetStateAction<string | undefined>>
  appConfig: AppConfig
}

export type SortBy = "creation_date" | "due_date" | "priority" | "done"
export type Order = "asc" | "desc"

export interface QueryParams {
  count?: number
  offset?: number
  sort_by?: SortBy
  order?: Order
  done?: boolean
  search?: string
}

export function DataTable({
  columns,
  data,
  sorting,
  setSorting,
  fetchTodos,
  statusFilter,
  setStatusFilter,
  searchString,
  setSearchString,
  appConfig,
}: DataTableProps) {
  const [selectedRow, setSelectedRow] = useState<TodoItem | null>(null);
  const [dialogOpen, setDialogOpen] = useState(false);

  const table = useReactTable({
    data,
    columns,
    state: {
      sorting,
    },
    onSortingChange: setSorting,
    getCoreRowModel: getCoreRowModel(),
  })

  const handleAddTodo = () => {
    setSelectedRow(null)
    setDialogOpen(true)
  }

  return (
    <div className="flex flex-col gap-4 h-full">

      <DataTableToolbar 
        fetchTodos={fetchTodos} 
        onAdd={handleAddTodo}
        statusFilter={statusFilter}
        setStatusFilter={setStatusFilter}
        searchString={searchString}
        setSearchString={setSearchString}
      />

      <div className="rounded-md border overflow-auto flex-1 min-h-0">
        <Table>
          <TableHeader>
            {table.getHeaderGroups().map((headerGroup) => (
              <TableRow key={headerGroup.id}>
                {headerGroup.headers.map((header) => {
                  return (
                    <TableHead key={header.id} colSpan={header.colSpan}>
                      {header.isPlaceholder
                        ? null
                        : flexRender(
                          header.column.columnDef.header,
                          header.getContext()
                        )}
                    </TableHead>
                  )
                })}
              </TableRow>
            ))}
          </TableHeader>
          <TableBody>
            {table.getRowModel().rows?.length ? (
              table.getRowModel().rows.map((row) => (
                <TableRow
                  key={row.id}
                  onClick={() => {
                    setSelectedRow(row.original);
                    setDialogOpen(true);
                  }}
                >
                  {row.getVisibleCells().map((cell) => (
                    <TableCell key={cell.id}>
                      {flexRender(cell.column.columnDef.cell, cell.getContext())}
                    </TableCell>
                  ))}
                </TableRow>
              ))
            ) : (
              <TableRow className="hover:bg-transparent">
                <TableCell colSpan={columns.length} className="h-24 text-center">
                  No results.
                </TableCell>
              </TableRow>
            )}
          </TableBody>
        </Table>
      </div>

      <TodoItemDialog 
        open={dialogOpen} 
        onOpenChange={setDialogOpen}
        todo={selectedRow}
        fetchTodos={fetchTodos}
        appConfig={appConfig}
      />
    </div>
  )
}