export default function CarCard({ isTaken }: { isTaken: boolean }) {
  const color = (isTaken: boolean): string => {
    return isTaken ? "bg-red-500" : "bg-green-500";
  };

  return (
    <div className="flex flew-row ">
      <div className={`${color(isTaken)} h-6 w-6 rounded`} />
      <p className="ml-2">{isTaken ? 'Taken' : 'Available'}</p>
    </div >
  );
}

