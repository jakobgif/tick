import { ColumnDef } from "@tanstack/react-table"
import { Checkbox } from "./ui/checkbox"
import { text } from "node:stream/consumers"
import { CheckCircle, Circle } from "lucide-react"

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
    header: "Title",
    cell: ({ row }) => <div className="w-[200px] truncate">{row.getValue("title")}</div>,
  },
  {
    accessorKey: "content",
    header: "Content",
    cell: ({ row }) => {
      return (
        <div className="flex">
          <span className="max-w-[400px] truncate font-medium">
            {row.getValue("content")}
          </span>
        </div>
      );
    },
  },
  {
    accessorKey: "done",
    header: "Status",
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
  },
  {
    accessorKey: "priority",
    header: "Priority",
  },
  {
    accessorKey: "due_date",
    header: "Due",
    size: 145,
    cell: info => {
      const date = new Date(info.getValue<number>() * 1000)

      return new Intl.DateTimeFormat("en-GB", {
        dateStyle: "short",
        timeStyle: "short",
        hour12: false,
      }).format(date)
    },
  },
]