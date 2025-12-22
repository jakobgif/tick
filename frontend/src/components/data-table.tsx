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
import { Dialog, DialogClose, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle, DialogTrigger } from "./ui/dialog";
import { Button } from "./ui/button";
import { useState } from "react";

interface DataTableProps<TData, TValue> {
  columns: ColumnDef<TData, TValue>[]
  data: TData[]
  sorting: SortingState
  setSorting: React.Dispatch<React.SetStateAction<SortingState>>
}

export type SortBy = "creation_date" | "due_date" | "priority"
export type Order = "asc" | "desc"

export interface QueryParams {
  count?: number
  offset?: number
  sort_by?: SortBy
  order?: Order
  done?: boolean
}

export function DataTable<TData, TValue>({
  columns,
  data,
  sorting,
  setSorting,
}: DataTableProps<TData, TValue>) {
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

  return (
    <div className="flex flex-col gap-4">

      <DataTableToolbar table={table} />

      <div className="rounded-md border overflow-auto max-h-[300px]">
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
              <TableRow>
                <TableCell colSpan={columns.length} className="h-24 text-center">
                  No results.
                </TableCell>
              </TableRow>
            )}
          </TableBody>
        </Table>
      </div>

      <Dialog open={dialogOpen} onOpenChange={setDialogOpen}>
        <DialogContent className="sm:max-w-[425px]">
          <DialogHeader>
            <DialogTitle>Edit Todo</DialogTitle>
            <DialogDescription>
              Edit your Todo here. Click save when you&apos;re
              done.
            </DialogDescription>
          </DialogHeader>

          <div className="flex flex-col gap-2">
            {/* Example for TodoItem fields */}
            <label>
              Title:
              <input
                className="w-full border rounded p-1"
                value={selectedRow.title}
                onChange={(e) =>
                  setSelectedRow({ ...selectedRow, title: e.target.value })
                }
              />
            </label>
            <label>
              Content:
              <textarea
                className="w-full border rounded p-1"
                value={selectedRow.content}
                onChange={(e) =>
                  setSelectedRow({ ...selectedRow, content: e.target.value })
                }
              />
            </label>
          </div>

          <DialogFooter>
            <DialogClose asChild>
              <Button variant="outline">Cancel</Button>
            </DialogClose>
              <Button type="submit">Save changes</Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </div>
  )
}