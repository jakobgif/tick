import { Table } from "@tanstack/react-table"
import { Input } from "./ui/input"
import { Button } from "./ui/button"
import { CircleFadingPlus } from "lucide-react";

export function DataTableToolbar<TData>({
  table,
}: {
  table: Table<TData>
}) {
  //const isFiltered = table.getState().columnFilters.length > 0

  return (
    <div className="flex items-center justify-between">
      <div className="flex flex-1 items-center gap-2">
        <Input
          placeholder="Filter tasks..."
          value={(table.getColumn("title")?.getFilterValue() as string) ?? ""}
          onChange={(event) =>
            table.getColumn("title")?.setFilterValue(event.target.value)
          }
          className="h-8 w-[150px] lg:w-[250px]"
        />
      </div>
      <div>
        <Button variant="outline" size="icon-sm" aria-label="add">
          <CircleFadingPlus />
        </Button>
      </div>
    </div>
  )
}