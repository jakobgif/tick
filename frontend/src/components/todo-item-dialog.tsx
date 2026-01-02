import { useEffect, useState } from "react"
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
import { priorities, TodoItem } from "./columns"
import { ChevronDownIcon, Icon, Trash2 } from "lucide-react"
import { toast } from "sonner"
import { invoke } from "@tauri-apps/api/core"
import { format } from "date-fns"

interface TodoDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  todo: TodoItem | null
  fetchTodos: () => Promise<void>
}

export function TodoItemDialog({ 
  open,
  onOpenChange,
  todo,
  fetchTodos
}: TodoDialogProps) {
  const [title, setTitle] = useState(todo?.title || "");
  const [content, setContent] = useState(todo?.content || "");
  const [done, setDone] = useState(todo?.done || false);
  const [priority, setPriority] = useState<number>(0);

  // const [dueDate, setDueDate] = useState<Date | undefined>(
  //   todo ? new Date(todo.due_date) : undefined
  // );
  // const [dueTime, setDueTime] = useState(todo ? formatTime(todo.due_date) : "10:30:00");
  // const [datePickOpen, setDatePickOpen] = useState(false);

  //init state when dialog opens or todo changes
  useEffect(() => {
    if (todo) {
      setTitle(todo.title)
      setContent(todo.content)
      setDone(todo.done)
      setPriority(todo.priority)
      //setDueDate(new Date(todo.due_date * 1000))
    } else {
      setTitle("")
      setContent("")
      setDone(false)
      setPriority(0)
      //setDueDate(undefined) //today
    }
  }, [todo, open])

  const handleSave = async () => {
    if (!title.trim()) {
      toast.error("Title is required");
      return;
    }

    //const dueDate = mergeDateTime(date, time);
    //const now = new Date();

    const newTodo: TodoItem = {
      id: todo?.id || 0,
      title,
      content,
      done,
      priority,
      creation_date: 0,
      due_date: 0,
      finish_date: 0,
    };

    try {
      onOpenChange(false);
      if (todo) {
        // Update existing todo
        await invoke<string>("update_todo", { todo: newTodo })
      } else {
        // Create new todo
        await invoke<string>("create_todo", { todo: newTodo })
      }
      fetchTodos();
    } catch (err) {
      toast.error("Failed to save todo: " + err);
    }
  };

  const handleDelete = async () => {
    try {
      onOpenChange(false);
      await invoke<string>("delete_todo", { todo })
      fetchTodos();
    } catch (err) {
      toast.error("Failed to delete todo: " + err);
    }
  }

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
          <Input type="text" value={title} placeholder={"drink water"} onChange={(e) => setTitle(e.target.value)}/>
        </Field>

        <Field>
          <FieldLabel>
            Description
          </FieldLabel>
          <Textarea
            value={content}
            onChange={(e) => setContent(e.target.value)}
            placeholder={"drink 3.7 liters of water today"}
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
                  checked={done}
                  onCheckedChange={(v) => setDone(Boolean(v))}
                />
              </Field>
            </div>
          )}

          <div>
            <Field orientation="horizontal">
              <FieldLabel>
                Priority
              </FieldLabel>
              <Select 
                value={priority.toString()}
                onValueChange={(v) => setPriority(Number(v))}
              >
                <SelectTrigger>
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  {[...priorities].reverse().map((p) => (
                    <SelectItem key={p.value} value={p.value.toString()}>
                      <div className="flex items-center gap-2">
                        <p.icon />
                        {p.label}
                      </div>
                    </SelectItem>
                  ))}
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
            {/* <Popover open={datePickOpen} onOpenChange={setDatePickOpen}>
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
            </Popover> */}
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

        {todo && (
          <div className="flex flex-col gap-1">
            <Field className="text-muted-foreground">
              <FieldLabel>
                Created on:{" "}
                {todo?.creation_date
                  ? new Intl.DateTimeFormat(undefined, {
                      month: "2-digit",
                      day: "2-digit",
                      year: "numeric",
                      hour: "2-digit",
                      minute: "2-digit",
                      second: "2-digit",
                      hour12: false,
                    }).format(new Date(todo.creation_date * 1000))
                  : "—"}
              </FieldLabel>
            </Field>

            <Field className="text-muted-foreground">
              <FieldLabel>
                Closed on:{" "}
                {todo?.finish_date
                  ? new Intl.DateTimeFormat(undefined, {
                      month: "2-digit",
                      day: "2-digit",
                      year: "numeric",
                      hour: "2-digit",
                      minute: "2-digit",
                      second: "2-digit",
                      hour12: false,
                    }).format(new Date(todo.finish_date * 1000))
                  : "—"}
              </FieldLabel>
            </Field>
          </div>
        )}        
        
        <DialogFooter>
          <div className="w-full flex flex-row justify-between">
            {todo ? (
              <Button variant="destructive" size="icon" onClick={handleDelete}><Trash2 /></Button>
            ) : (
              <div></div>
            )}
            <div className="flex flex-row gap-2">
              <DialogClose asChild>
                <Button variant="outline">Cancel</Button>
              </DialogClose>
              <Button type="submit" onClick={handleSave}>{todo? "Save changes" : "Save"}</Button>
            </div>
          </div>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}