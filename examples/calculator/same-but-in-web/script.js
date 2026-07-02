class Calculator {
  constructor() {
    this.first = null;
    this.second = null;
    this.result = null;

    this.op = null;
    this.current = 0;

    document.querySelectorAll(".button-nbr").forEach((button) => {
      button.addEventListener("click", () => {
        this.add_nbr(button.dataset.nbr);
        this.update_ui();
      });
    });

    document.querySelectorAll(".button-op").forEach((button) => {
      button.addEventListener("click", () => {
        this.set_op(button.dataset.op);
        this.update_ui();
      });
    });

    document
      .querySelector(".button-calculate")
      .addEventListener("click", () => {
        this.calculate();
        this.update_ui();
      });

    document.querySelector(".button-clear").addEventListener("click", () => {
      this.first = null;
      this.second = null;
      this.result = null;
      this.current = null;
      this.update_ui();
    });
  }
  add_nbr(nbr) {
    if (this.current === 0) {
      if (!this.first) {
        this.first = "";
      }
      this.first += nbr;
    } else {
      if (!this.second) {
        this.second = "";
      }
      this.second += nbr;
    }
  }

  set_op(toOp) {
    this.op = toOp;
    this.current = (this.current + 1) % 2;
  }

  calculate() {
    if (this.first && this.second && this.op) {
      let a = Number(this.first);
      let b = Number(this.second);
      switch (this.op) {
        case "+":
          this.result = a + b;
          break;
        case "-":
          this.result = a - b;
          break;
        case "*":
          this.result = a * b;
          break;
        case "/":
          this.result = a / b;
          break;
      }
    }
  }

  update_ui() {
    document.getElementById("first-nbr").textContent = this.first ?? "0";
    document.getElementById("op").textContent = this.op ?? "+";
    document.getElementById("second-nbr").textContent = this.second ?? "0";
    document.getElementById("result-nbr").textContent = this.result ?? "0";
  }
}

new Calculator();
