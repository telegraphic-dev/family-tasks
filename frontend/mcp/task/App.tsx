import { type FC } from "react";
import { useTask, type UseTaskApi } from "@api/family_tasks/v1/family_tasks_rbt_react";
import css from "./App.module.css";
const Detail: FC<{ state: UseTaskApi }> = ({ state }) => { const { response } = state.useDetails(); return <div className={css.container}><p className={css.label}>{response?.status}</p><h1>{response?.title}</h1><p>{response?.notes || "No notes"}</p><p className={css.data}>{response?.dueDate ? `Due ${response.dueDate}` : "No due date"}</p></div>; };
export const ShowApp: FC = () => { const { task, isLoading } = useTask(); if (isLoading || !task) return <div className={css.container}>Loading task…</div>; return <Detail state={task} />; };
