import CarSection from "./components/carSection";
import Header from "./components/header";
export default function Home() {
  return (
    <main className="flex min-h-screen flex-col items-center bg-gradient-to-r from-indigo-500 to-purple-500">
      <Header />
      <CarSection />
    </main>
  );
}
