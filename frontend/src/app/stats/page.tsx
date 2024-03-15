import Header from "../components/header";
import StatCard from "../components/stats/statCard";

export default function Stats() {
  return (
    <main className="flex min-h-screen flex-col items-center bg-gradient-to-r from-indigo-500 to-purple-500">
      <Header />
      <div className="flex flex-row mt-5 w-4/5 justify-evenly">
        <StatCard name="Slots taken" value="3/6" />
        <StatCard name="Average park time" value="3h 16m" />
        <StatCard name="Longest today" value="5h 27m" />
      </div>
    </main>
  )
}
