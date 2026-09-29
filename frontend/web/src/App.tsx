import { useState, type FC } from "react";
import { useSignIn, useSignOut } from "@reboot-dev/reboot-react";
import { useHousehold, useUser, type UseUserApi } from "@api/family_tasks/v1/family_tasks_rbt_react";

const HouseholdCard: FC<{ id: string }> = ({ id }) => {
  const household = useHousehold({ id });
  const { response, isLoading, aborted } = household.useBoard();
  const [title, setTitle] = useState("");
  const [error, setError] = useState("");
  const addTask = async () => {
    const { aborted } = await household.addTask({ title });
    if (aborted) setError(aborted.message); else setTitle("");
  };
  if (isLoading && !response) return <section className="card">Loading household…</section>;
  if (aborted || !response) return <section className="card error">{aborted?.message ?? "Unable to open household."}</section>;
  return <section className="card"><p className="eyebrow">Shared board</p><h2>{response.name}</h2><p className="meta">{response.memberIds.length} family members</p><div className="row"><label className="meta" htmlFor={`task-${id}`}>New task</label><input className="field" id={`task-${id}`} value={title} onChange={(e) => setTitle(e.target.value)} placeholder="e.g. Empty the dishwasher" /><button className="action" disabled={!title.trim()} onClick={() => void addTask()}>Add task</button></div>{error && <p className="error">{error}</p>}<div aria-label={`${response.name} tasks`}>{response.tasks.length === 0 ? <p className="empty">Nothing on the board. Nice.</p> : response.tasks.map((task) => <article className={`task ${task.status === "completed" ? "done" : ""}`} key={task.taskId}><span>{task.title}<br/><small className="meta">{task.assigneeId ? `Assigned: ${task.assigneeId}` : "Unassigned"}{task.dueDate ? ` · due ${task.dueDate}` : ""}</small></span><span className="meta">{task.status}</span></article>)}</div></section>;
};

const SignedIn: FC<{ user: UseUserApi; onSignOut: () => void }> = ({ user, onSignOut }) => {
  const { response, isLoading } = user.useListHouseholds();
  const [name, setName] = useState("");
  const create = async () => { await user.createHousehold({ name }); setName(""); };
  return <main className="shell"><header className="mast"><div><p className="eyebrow">Shared household work</p><h1 className="brand">Family Tasks</h1><p className="subtitle">A calm, durable board for the little jobs that otherwise become everybody’s problem.</p></div><button className="sign" onClick={onSignOut}>Sign out</button></header><section className="card" style={{ marginTop: 26 }}><h2>Create a household</h2><div className="row"><label className="meta" htmlFor="household-name">Household name</label><input className="field" id="household-name" value={name} onChange={(e) => setName(e.target.value)} placeholder="Home Team"/><button className="action" disabled={!name.trim()} onClick={() => void create()}>Create household</button></div></section><div className="grid">{isLoading && !response ? <p>Loading your boards…</p> : (response?.households ?? []).map((household) => <HouseholdCard key={household.householdId} id={household.householdId} />)}</div></main>;
};

export default function App() {
  const { user, isLoading } = useUser();
  const signIn = useSignIn(); const signOut = useSignOut();
  if (isLoading) return <main className="shell">Checking your session…</main>;
  if (!user) return <main className="shell"><header className="mast"><div><p className="eyebrow">Shared household work</p><h1 className="brand">Family Tasks</h1><p className="subtitle">One place for the jobs that make a home run.</p></div></header><section className="card" style={{ marginTop: 26 }}><h2>Bring the family in sync.</h2><button className="action" onClick={() => void signIn()}>Sign in</button></section></main>;
  return <SignedIn user={user} onSignOut={() => void signOut()} />;
}
