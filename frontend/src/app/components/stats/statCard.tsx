export default function StatCard({ name, value }: { name: string, value: string | number }) {
  return (
    <div className="flex flex-col w-40 h-48 border-solid rounded-lg justify-evenly items-center backdrop-brightness-75">
      <p className="font-bold text-center text-2xl">{name}</p>
      <p>{value}</p>
    </div>
  )
}
