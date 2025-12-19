import { ColumnDef } from "@tanstack/react-table"
import { CheckCircle, Circle } from "lucide-react"
import { DataTableColumnHeader } from "./data-table-column-header"

//this type is based on the backend data structure
export type TodoItem = {
  id: number
  title: string
  content: string
  done: boolean
  priority: number
  creation_date: number //epoch seconds
  due_date: number //epoch seconds
  finish_date: number //epoch seconds
}

export const columns: ColumnDef<TodoItem>[] = [
  {
    accessorKey: "title",
    header: ({ column }) => (
      <DataTableColumnHeader column={column} title="Task" />
    ),
    cell: ({ row }) => <div className="w-[100px] truncate">{row.getValue("title")}</div>,
    enableSorting: false,
  },
  {
    accessorKey: "content",
    header: ({ column }) => (
      <DataTableColumnHeader column={column} title="Description" />
    ),
    cell: ({ row }) => {
      return (
        <div className="flex">
          <span className="max-w-[300px] truncate font-medium">
            {row.getValue("content")}
          </span>
        </div>
      );
    },
    enableSorting: false,
  },
  {
    accessorKey: "done",
    header: ({ column }) => (
      <DataTableColumnHeader column={column} title="Status" />
    ),
    cell: ({ row }) => {
      const done = row.getValue<boolean>("done")
      const status = done
        ? { label: "Done", icon: CheckCircle }
        : { label: "Open", icon: Circle }

      const Icon = status.icon
      return (
        <div className="flex items-center gap-2">
          <Icon className="h-4 w-4 text-muted-foreground" />
          <span>{status.label}</span>
        </div>
      )
    },
    enableSorting: true,
  },
  {
    accessorKey: "priority",
    header: ({ column }) => (
      <DataTableColumnHeader column={column} title="Priority" />
    ),
    enableSorting: true,
  },
  {
    accessorKey: "due_date",
    header: ({ column }) => (
      <DataTableColumnHeader column={column} title="Due" />
    ),
    cell: info => {
      const date = new Date(info.getValue<number>() * 1000)

      return new Intl.DateTimeFormat("en-GB", {
        dateStyle: "short",
        timeStyle: "short",
        hour12: false,
      }).format(date)
    },
    enableSorting: true,
  },
]