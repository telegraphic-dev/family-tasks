import { type FC } from "react";
import { useUser, type UseUserApi } from "@api/family_tasks/v1/family_tasks_rbt_react";
import css from "./App.module.css";
const Dashboard: FC<{ state: UseUserApi }> = ({ state }) => { const { response } = state.useListHouseholds(); return <div className={css.container}><h1>Family Tasks</h1>{(response?.households ?? []).map((h) => <div className={css.data} key={h.householdId}>{h.name}</div>)}{response?.households.length === 0 && <p>No household yet. Ask me to create one.</p>}</div>; };
export const ShowDashboardApp: FC = () => { const { user, isLoading } = useUser(); if (isLoading || !user) return <div className={css.container}>Loading family boards…</div>; return <Dashboard state={user} />; };
