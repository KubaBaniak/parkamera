'use client'

import { FaArrowDown, FaArrowUp } from "react-icons/fa";
import CarDetails from './cardDetails';
import { useState } from "react";

export default function CarCard({ isTaken, id }: { isTaken: boolean, id: number }) {
  const [show, toggleShow] = useState(false);

  const color = (isTaken: boolean): string => {
    return isTaken ? "bg-red-500" : "bg-green-500";
  };

  const handleToggle = () => {
    toggleShow(!show);
  };

  return (
    <div>
      <div
        className="flex flex-row items-center justify-center gap-2 w-auto cursor-pointer
        hover:bg-violet-600 active:bg-violet-700 rounded-lg p-3"
        onClick={handleToggle}
      >
        <p>No. {id + 1}</p>
        <div className={`${color(isTaken)} h-6 w-6 rounded`} />
        {isTaken ? (
          <>
            <span className="animate-pulse">Taken</span>
            {show ? <FaArrowUp /> : <FaArrowDown />}
          </>
        ) : (
          <span>Available</span>
        )}
      </div>
      {isTaken && show && <CarDetails />}
    </div>
  );
}
