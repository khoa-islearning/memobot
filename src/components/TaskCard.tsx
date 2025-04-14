import { useState } from "react";
import { Task } from "../models/task";
import { FaTrashAlt } from "react-icons/fa";
import { invoke } from "@tauri-apps/api/core";
type TaskCardProps = {
  task: Task;
  updateFunction: Function;
};

const TaskCard = ({ task, updateFunction }: TaskCardProps) => {
  const [showInfo, setShowInfo] = useState<Boolean>(false);
  const handleRate = async (rating: number) => {
    await invoke("rate_task", { id: task.id, rating: rating });
    updateFunction();
  };
  return (
    <div className="task-card">
      <div
        style={{ cursor: "pointer" }}
        onClick={() =>
          setShowInfo((e) => {
            return !e;
          })
        }
      >
        {task.name}
      </div>
      {showInfo && (
        <div>
          <a href={task.url} target="_blank" rel="noopener noreferrer">
            {task.url}
          </a>
          <p>Due: {task.due_date}</p>
          <div
            style={{
              display: "flex",
              flexDirection: "row",
              gap: "4px",
              alignItems: "center",
              justifyContent: "center",
            }}
          >
            <button onClick={() => handleRate(1)}>easy</button>
            <button onClick={() => handleRate(2)}>hard</button>
            <button onClick={() => handleRate(3)}>again</button>
            <button
              onClick={async () => {
                await invoke("delete_task", { id: task.id });
                updateFunction();
              }}
            >
              <FaTrashAlt color="red" />
            </button>
          </div>
        </div>
      )}
    </div>
  );
};

export default TaskCard;
