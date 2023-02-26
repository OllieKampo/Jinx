from build.example import add, arg_first_where, Mult

from concurrent.futures import ThreadPoolExecutor
import time
import numpy as np

print(add(1, 2))
m = Mult(2)
print(m.multiply(2))
print(m.multiply_vector([1, 2, 3]))
print(m.multiply_tuple((1, 2, 3)))

a = np.array([1, 2, 3])
# print(first_where(a, lambda x: x > 1))
print(arg_first_where(a, lambda x: x > 1))

start_time = time.time()

with ThreadPoolExecutor(max_workers=4) as executor:
    iter_ = executor.map(lambda x: m.function_concurrent_callable(), [None]*4)
    for r in iter_:
        print(r)

print("--- %s seconds ---" % (time.time() - start_time))

from build.example import simulate_control

from control.pid import PIDController
from control.systems import InvertedPendulumSystem

pend_system = InvertedPendulumSystem()
controller = PIDController(15.94104423478139, 1.0806148906694266, 5.159623499281683)

a = simulate_control(pend_system, controller, 100, 0.1, True, True, 1, 1)
print(a)