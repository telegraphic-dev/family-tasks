import { type FC } from "react";
import { useHousehold, type UseHouseholdApi } from "@api/family_tasks/v1/family_tasks_rbt_react";
import css from "./App.module.css";
const Board: FC<{ state: UseHouseholdApi }> = ({ state }) => { const { response } = state.useBoard(); return <div className={css.container}><h1>{response?.name ?? "Household"}</h1>{(response?.tasks ?? []).map((task) => <div className={css.data} key={task.taskId}><strong>{task.title}</strong> — {task.status}</div>)}</div>; };
export const ShowBoardApp: FC = () => { const { household, isLoading } = useHousehold(); if (isLoading || !household) return <div className={css.container}>Loading board…</div>; return <Board state={household} />; };
