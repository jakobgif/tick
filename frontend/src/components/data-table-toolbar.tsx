import { Input } from "./ui/input"
import { Button } from "./ui/button"
import { Check, CheckCircle, Circle, CircleFadingPlus, ListRestart, PlusCircle, X } from "lucide-react";
import { ButtonGroup } from "./ui/button-group";
import { Popover, PopoverContent, PopoverTrigger } from "./ui/popover";
import { Command, CommandGroup, CommandItem, CommandList } from "./ui/command";
import { useState } from "react";
import { Separator } from "./ui/separator";

export function DataTableToolbar({
  fetchTodos,
  onAdd,
  statusFilter,
  setStatusFilter,
  searchString,
  setSearchString,
}: {
  fetchTodos: () => Promise<void>
  onAdd: () => void
  statusFilter: boolean | undefined
  setStatusFilter: (v: boolean | undefined) => void
  searchString: string | undefined
  setSearchString: (v: string | undefined) => void
}) {
  const [popupOpen, setPopupOpen] = useState(false);

  return (
    <div className="flex items-center justify-between">
      <div className="flex flex-1 items-center gap-2">
        <Input
          placeholder="Filter tasks..."
          value={searchString ?? ""}
          onChange={(e) => {
            const value = e.target.value
            setSearchString(value.trim() === "" ? undefined : value)
          }}
          className="h-8 w-[150px] lg:w-[250px]"
        />

        <Button variant="outline" size="icon-sm" onClick={() => setSearchString(undefined)} disabled={searchString === undefined}>
          <X />
        </Button>

        <Popover open={popupOpen} onOpenChange={setPopupOpen}>
          <PopoverTrigger asChild>
            <Button variant="outline" size="sm" className="h-8 border-dashed">
              <PlusCircle />
              Status
              {statusFilter !== undefined && (
                <span className="text-muted-foreground">
                  ({statusFilter ? "Done" : "Open"})
                </span>
              )}
            </Button>
          </PopoverTrigger>
          <PopoverContent className="w-[200px] p-0" align="start">
            <Command>
              <CommandList>
                <CommandGroup>
                  <CommandItem
                    onSelect={() => {
                      setStatusFilter(true);
                      setPopupOpen(false);
                    }}
                    className="flex items-center justify-between"
                  >
                    <div className="flex items-center gap-2">
                      <CheckCircle className="size-4" />
                      <span>Done</span>
                    </div>
                    {statusFilter === true && <Check className="text-primary size-4" />}
                  </CommandItem>

                  <CommandItem
                    onSelect={() => {
                      setStatusFilter(false);
                      setPopupOpen(false);
                    }}
                    className="flex items-center justify-between"
                  >
                    <div className="flex items-center gap-2">
                      <Circle className="size-4" />
                      <span>Open</span>
                    </div>
                    {statusFilter === false && <Check className="text-primary size-4" />}
                  </CommandItem>

                  <Separator className="my-1"></Separator>

                  <CommandItem
                    onSelect={() => {
                      setStatusFilter(undefined);
                      setPopupOpen(false);
                    }}
                    className="justify-center text-muted-foreground"
                  >
                    <span>Clear Filter</span>
                  </CommandItem>
                </CommandGroup>
              </CommandList>
            </Command>
          </PopoverContent>
        </Popover>
      </div>
      
      <ButtonGroup>
        <Button variant="outline" size="icon-sm" aria-label="add" onClick={onAdd}>
          <CircleFadingPlus />
        </Button>
        <Button variant="outline" size="icon-sm" aria-label="add" onClick={fetchTodos}>
          <ListRestart />
        </Button>
      </ButtonGroup>
    </div>
  )
}