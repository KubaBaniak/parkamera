import CarCard from "./carCard";

export default function CarSection() {
  const carSlots = [true, true, false, true, false, false];

  return (
    <div className="flex flex-col mt-9 flex-grow-0 gap-10 flex-wrap justify-center">
      {carSlots.map((isTaken, index) => (
        <CarCard key={index.toString()} id={index} isTaken={isTaken} />
      ))}
    </div>
  );
}
