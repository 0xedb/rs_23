class Home {

    static Value = 53423;
    
    static {
        console.log("welcome home")
        this.Value = 0xedb;
    }
}

const h = new Home();

console.log(h, h.Value, Home.Value)