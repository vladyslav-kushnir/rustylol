import logo from './logo.svg';
import './App.css';
import CommandsList from './CommandsList';

function App() {
  return (
    <div className="App">
      <header className="App-header">
        Commands list
      </header>
      <div>
        <CommandsList />
      </div>
    </div>
  );
}

export default App;
