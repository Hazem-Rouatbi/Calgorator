const { invoke } = window.__TAURI__.tauri;
//global variables (decided to make most of them global since it's a small app)
let num_btns;
//for all he eaquations
let symbol_btns;
let C_btn;
let first_inp;
let display;
let variable_list;
let symbols_list;
let current_equation;
let current_variable;
let state;
let current_input_div;
let symbol_disp;
const MAX_EQUATION_LENGTH = 2;

async function greet() {
  greetMsgEl.textContent = await invoke("greet", { name: "" });
}

function check_key_press(event) {
  const key = event.key
  console.log(current_variable);
  if(max_equation_lenth())
  { if (key == '/' || key == '-' || key == '+' || key == '*') {
    if (var_not_null()) {
      add_varaiable();
      state = "equa"
      current_equation = key
    }
    if (state == "equa") {
      current_equation = key
    }
    symbol_disp.textContent = current_equation

  }
}
  if (key == 'Enter') {
    if (symbols_list.length != (variable_list.length-1))
      add_varaiable()
    console.log(variable_list)
    console.log(symbols_list);
    submit_equation()
  }
  if (state == "equa") {
    state = "num"
    symbols_list.push(current_equation)
    console.log(symbols_list);
  }

}
async function submit_equation() {
  const result = await invoke("calculate", { vars : variable_list,symbs : symbols_list });
  symbols_list = [];
  variable_list = [];
  current_variable = result.substr(0,result.indexOf('.')+6);;
  current_input_div.value = current_variable
  symbol_disp.textContent = "="
}

function add_varaiable() {
  variable_list.push(current_variable);
  console.log(variable_list);
  current_variable = ''
  current_input_div.value = ''
}

let check_val = (val) => {
  if (val != undefined) {
    var rgx = /^[0-9][0-9]{0,10}(\.?[0-9]{0,8})$/;
    if (var_max_min(val)) {
      return val.match(rgx);
    }
  }

  return false
}

let max_equation_lenth = () => {return (symbols_list.length <= MAX_EQUATION_LENGTH)}
let var_max_min = (val) => { return (Number.parseFloat(val) < 1000000000000) && (Number.parseFloat(val) > -1000000000000) }
let var_not_null = () => { return (current_variable != '' && current_variable != null && current_variable != undefined) }

window.addEventListener("DOMContentLoaded", () => {
  //this variable is for switching between inputting symbols and numbers
  state = 'num'
  //loading variables
  variable_list = []
  symbols_list = []
  //loading buttons
  num_btns = document.querySelectorAll('.num-btn')
  symbol_btns = document.querySelectorAll(".equa-btn")
  C_btn = document.querySelector(".C-btn")
  symbol_disp = document.querySelector(".symbol")


  //loading display and first input
  display = document.querySelector(".display")
  first_inp = document.querySelector("#number-0")

  current_input_div = first_inp;
  //focus the input when window is brought into focus
  window.addEventListener('focus', () => {
    current_input_div.focus();
  })
  //reset everything
  C_btn.addEventListener("click", () => {
    state = "num"
    current_variable = null
    current_input_div = first_inp
    current_input_div.value = current_variable
    variable_list = []
    symbols_list = []
    display.append(current_input_div)
    current_input_div.focus()
    symbol_disp.textContent = ''
  })

  //btn event for the symbols
  current_input_div.addEventListener("input", () => {
    current_variable = (check_val(current_input_div.value)) ? current_input_div.value : current_variable;
    current_input_div.value = current_variable
  })
  symbol_btns.forEach(btn => {
    btn.addEventListener("click", () => {
      const added_symbol = btn.getAttribute("equa")
      const keypress = new KeyboardEvent("keyup", { key: added_symbol, bubbles: true })
      btn.dispatchEvent(keypress)
    })
  });


  //btn event for the numbers (this could be added to the symnols btns)
  num_btns.forEach(btn => {
    btn.addEventListener("click", function () {
      const added_num = btn.getAttribute('num')
      let new_value = ""
      if (var_not_null()) {
        new_value = current_variable.concat(added_num);
      }
      else
        new_value = added_num
      current_variable = (check_val(new_value)) ? new_value : current_variable;
      current_input_div.value = current_variable
    });
  })

  document.body.addEventListener("keyup", check_key_press)
  document.body.addEventListener("click", () => { current_input_div.focus() })
});
