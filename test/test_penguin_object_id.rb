# frozen_string_literal: true

require "test_helper"

class TestPenguinObjectId < Minitest::Test
  def test_from_time_unique
    t = Time.now - 1000
    100.times.reduce(Penguin::ObjectId.from_time(t, unique: true)) do |prev, _|
      id = Penguin::ObjectId.from_time(t, unique: true)
      assert_equal t.floor, id.timestamp
      assert_operator prev.counter, :<, id.counter
      assert_equal prev.machine_id, id.machine_id
      assert_operator prev.counter, :<, id.counter
      assert_operator prev, :<, id
      id
    end

  end

  def test_from_time_not_unique
    t = Time.now - 1000
    100.times.reduce(Penguin::ObjectId.from_time(t, unique: false)) do |prev, _|
      id = Penguin::ObjectId.from_time(t, unique: false)
      assert_equal t.floor, id.timestamp
      assert_equal 0, id.counter
      assert_equal prev.machine_id, id.machine_id
      assert_equal prev, id
      id
    end
  end

  def test_new
    100.times.reduce(Penguin::ObjectId.new) do |prev, _|
      id = Penguin::ObjectId.new
      assert_operator prev.counter, :<, id.counter
      assert_equal prev.machine_id, id.machine_id
      id
    end
  end

  def test_from_string
    strings = [
      '64c13ab08edf48a008793cac',
      '4e4d66343b39b68407000001',
    ]

    strings.each do |string|
      id = Penguin::ObjectId.from_string(string)
      assert_equal string, id.to_s
    end
  end
end
