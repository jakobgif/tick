import { useState } from "react"
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "./ui/dialog"
import { Button } from "./ui/button"
import { Input } from "./ui/input"
import { Textarea } from "./ui/textarea"
import { Checkbox } from "./ui/checkbox"
import { Field, FieldLabel } from "./ui/field"
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "./ui/select"
import { Popover, PopoverContent, PopoverTrigger } from "./ui/popover"
import { Calendar } from "./ui/calendar"
import { TodoItem } from "./columns"
import { ChevronDownIcon } from "lucide-react"

interface TodoDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  todo: TodoItem | null
}

export function TodoItemDialog({ 
  open,
  onOpenChange,
  todo,
}: TodoDialogProps) {
  const [DatePickOpen, setDatePickOpen] = useState(false)
  const [date, setDate] = useState<Date | undefined>(undefined)

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-[425px]">
        <DialogHeader>
          <DialogTitle>{todo ? "Edit Todo" : "Create Todo"}</DialogTitle>
          <DialogDescription>
            {todo ? "Edit your Todo here. " : "Fill the fields to create a new Todo. "}
            Click save when you&apos;re done.
          </DialogDescription>
        </DialogHeader>

        <Field>
          <FieldLabel>
            Title
          </FieldLabel>
          <Input type="text" placeholder={todo? todo.title : "drink water"} />
        </Field>

        <Field>
          <FieldLabel>
            Description
          </FieldLabel>
          <Textarea
            placeholder={todo? todo.content : "drink 3.7 liters of water today"}
            className="resize-none"
          />
        </Field>

        <div className="flex flex-row justify-start items-center gap-10">
          {todo && (
            <div>
              <Field orientation="horizontal">
                <FieldLabel>
                  Done
                </FieldLabel>
                <Checkbox
                  checked={todo?.done || false}
                />
              </Field>
            </div>
          )}

          <div>
            <Field orientation="horizontal">
              <FieldLabel>
                Priority
              </FieldLabel>
              <Select defaultValue="">
                <SelectTrigger>
                  <SelectValue placeholder="Low" />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="Low">Low</SelectItem>
                  <SelectItem value="Medium">Medium</SelectItem>
                  <SelectItem value="High">High</SelectItem>
                </SelectContent>
              </Select>
            </Field>
          </div>
        </div>

        <div className="flex flex-row justify-start items-center gap-3">
          <Field>
            <FieldLabel>
              Due Date
            </FieldLabel>
            <Popover open={DatePickOpen} onOpenChange={setDatePickOpen}>
              <PopoverTrigger asChild>
                <Button
                  variant="outline"
                  id="date-picker"
                  className="w-32 justify-between font-normal"
                >
                  {date ? date.toLocaleDateString() : "Select date"}
                  <ChevronDownIcon />
                </Button>
              </PopoverTrigger>
              <PopoverContent className="w-auto overflow-hidden p-0" align="start">
                <Calendar
                  mode="single"
                  selected={date}
                  captionLayout="dropdown"
                  onSelect={(date) => {
                    setDate(date)
                    setDatePickOpen(false)
                  }}
                />
              </PopoverContent>
            </Popover>
          </Field>

          <Field>
            <FieldLabel>
              Time
            </FieldLabel>
            <Input
              type="time"
              id="time-picker"
              step="1"
              defaultValue="10:30:00"
              className="bg-background appearance-none [&::-webkit-calendar-picker-indicator]:hidden [&::-webkit-calendar-picker-indicator]:appearance-none"
            />
          </Field>
        </div>
          
        <DialogFooter>
          <DialogClose asChild>
            <Button variant="outline">Cancel</Button>
          </DialogClose>
          <Button type="submit">{todo? "Save changes" : "Save"}</Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}