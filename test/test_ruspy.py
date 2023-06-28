import os
import unittest

import ruspy as rp
import numpy as np

class TestRusPy(unittest.TestCase):
  def test_array(self):
    r_array = rp.array([1, 2, 3, 4])
    n_array = np.array([1, 2, 3, 4])
    for i in range(4):
      self.assertEqual(r_array[i], n_array[i])

  def test_fft(self):
    r_array = rp.array([1, 2, 3, 4])
    n_array = np.array([1, 2, 3, 4])
    r_fft = rp.fft(r_array)
    n_fft = np.fft.fft(n_array)
    self.assertEqual(len(r_fft), len(n_fft))
    for i in range(len(r_fft)):
      self.assertEqual(r_fft[i], n_fft[i])

if __name__ == "__main__":
  unittest.main()