import Icon from "/icon.svg";

function App() {
    return (
        <>
            <img src={Icon} alt="the logo (a muscle)" />
            <h1>GymAnalyzer</h1>
            <p>
                <input type="file" />
            </p>
            <p>
                <button>Analysieren</button>
            </p>
        </>
    );
}

export default App;
