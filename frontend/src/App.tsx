import { useEffect, useState } from 'react';
import './App.css';

// Define Types
interface Todo {
  id: string;
  title: string;
  completed: boolean;
}

function App() {
  const [todos, setTodos] = useState<Todo[]>([]);
  const [newTodoTitle, setNewTodoTitle] = useState("");
  const [isAdding, setIsAdding] = useState(false); // Controls input visibility

  const API_URL = "http://localhost:3000/todos";

  // 1. Fetch Todos
  const fetchTodos = async () => {
    try {
      const res = await fetch(API_URL);
      const data = await res.json();
      // Sort: Completed items go to bottom
      const sorted = data.sort((a: Todo, b: Todo) => Number(a.completed) - Number(b.completed));
      setTodos(sorted);
    } catch (err) {
      console.error("Failed to fetch todos:", err);
    }
  };

  useEffect(() => {
    fetchTodos();
  }, []);

  // 2. Add Todo
  const handleAddTodo = async () => {
    if (!newTodoTitle.trim()) return;
    
    await fetch(API_URL, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ title: newTodoTitle })
    });
    
    setNewTodoTitle("");
    setIsAdding(false);
    fetchTodos();
  };

  // 3. Toggle Complete
  const toggleTodo = async (id: string, currentStatus: boolean) => {
    // Optimistic update for UI responsiveness
    setTodos(todos.map(t => t.id === id ? { ...t, completed: !currentStatus } : t));

    await fetch(`${API_URL}/${id}`, {
      method: "PATCH",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ completed: !currentStatus })
    });
    fetchTodos();
  };

  // 4. Delete Todo
  const deleteTodo = async (id: string) => {
    await fetch(`${API_URL}/${id}`, { method: "DELETE" });
    fetchTodos();
  };

  return (
    <div className="app-container">
      {/* Top Title */}
      <h1 className="main-title">RUST TO DO</h1>

      {/* Central Card */}
      <div className="todo-card">
        
        {/* Header Row */}
        <div className="card-header">
          <h2>TODO'S</h2>
          <button 
            className="add-btn" 
            onClick={() => setIsAdding(!isAdding)}
          >
            {isAdding ? "Close" : "+ Add new task"}
          </button>
        </div>

        {/* Input Area (Visible only when clicking 'Add new task') */}
        {isAdding && (
          <div className="input-row">
            <input 
              className="task-input"
              value={newTodoTitle} 
              onChange={(e) => setNewTodoTitle(e.target.value)}
              placeholder="Enter task details..."
              onKeyDown={(e) => e.key === 'Enter' && handleAddTodo()}
              autoFocus
            />
            <button className="add-btn" onClick={handleAddTodo}>Save</button>
          </div>
        )}

        {/* Scrollable List */}
        <ul className="todo-list">
          {todos.length === 0 && !isAdding && (
            <p style={{ textAlign: "center", color: "#555", marginTop: "2rem" }}>
              No tasks yet. Enjoy your day!
            </p>
          )}

          {todos.map((todo) => (
            <li key={todo.id} className="todo-item">
              <input 
                type="checkbox" 
                checked={todo.completed} 
                onChange={() => toggleTodo(todo.id, todo.completed)}
                style={{ cursor: "pointer", width: "18px", height: "18px" }}
              />
              <span className={`todo-text ${todo.completed ? 'completed' : ''}`}>
                {todo.title}
              </span>
              <button 
                className="delete-btn"
                onClick={() => deleteTodo(todo.id)}
                title="Delete Task"
              >
                &times;
              </button>
            </li>
          ))}
        </ul>
      </div>
    </div>
  );
}

export default App;