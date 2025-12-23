import { ColumnDef } from "@tanstack/react-table"
import { ArrowDown, ArrowRight, ArrowUp, CheckCircle, Circle } from "lucide-react"
import { DataTableColumnHeader } from "./data-table-column-header"
import { invoke } from "@tauri-apps/api/core"
import { toast } from "sonner"

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

const priorities = [
  {
    label: "Low",
    value: 0,
    icon: ArrowDown,
  },
  {
    label: "Medium",
    value: 100,
    icon: ArrowRight,
  },
  {
    label: "High",
    value: 200,
    icon: ArrowUp,
  },
]

export const columns = (fetchTodos: () => Promise<void>): ColumnDef<TodoItem>[] => [
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

      const handleClick = async (e: React.MouseEvent) => {
        //make sure to not trigger row onclick
        e.stopPropagation()

        const id = row.original.id

        try {
          await invoke<string>("toggle_todo_status", { id })
          await fetchTodos()
        } catch (err: any) {
          toast.error(err.toString())
        }
      }

      return (
        <div className="flex items-center gap-2" onClick={handleClick}>
          <Icon className="h-4 w-4 text-muted-foreground"/>
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
    cell: ({ row }) => {
      const priority = priorities.find(
        (priority) => priority.value === row.getValue("priority")
      )

      if (!priority) {
        return (
          <span>{row.getValue("priority")}</span>
        )
      }

      return (
        <div className="flex items-center gap-2">
          {priority.icon && (
            <priority.icon className="text-muted-foreground size-4" />
          )}
          <span>{priority.label}</span>
        </div>
      )
    },
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